use std::ffi::{CStr, CString};

use futures::TryStreamExt;
use mongodb::{Collection, Database};
use mongodb::bson::{Bson, Document};
use rglua::lua::{lua_pushboolean, lua_setmetatable, luaL_checkstring, luaL_getmetatable, LuaState};
use rglua::prelude::{lua_gettop, lua_istable, lua_newtable, lua_next, lua_pop, lua_pushnil, lua_pushnumber, lua_pushstring, lua_rawseti, lua_settable, lua_tonumber, lua_tostring, lua_type};

use crate::logger::{log, LogLevel};
use crate::mongo::MONGO_WORKER;
use crate::utils::luautils::{read_userdata, write_userdata};

const LUA_TNUMBER: i32 = 3;
const LUA_TSTRING: i32 = 4;
const LUA_TTABLE: i32 = 5;

unsafe fn get_table_key(l: LuaState, key_type: i32) -> Result<String, String> {
    if key_type == LUA_TSTRING {
        let key_ptr = lua_tostring(l, -2);
        if key_ptr.is_null() {
            lua_pop(l, 1);
            return Err("Invalid key type".to_string());
        }
        return Ok(CStr::from_ptr(key_ptr).to_str().unwrap().to_string());
    }

    if key_type == LUA_TNUMBER {
        let key = lua_tonumber(l, -2);
        return Ok(key.to_string());
    }

    Err("Table key must be a string or number".to_string())
}

fn lua_table_to_bson(l: LuaState, index: i32) -> Result<Document, String> {
    #[allow(unused_unsafe)]
    if !unsafe { lua_istable(l, index) } {
        return Err("Expected a table".to_string());
    }

    let mut doc = Document::new();
    unsafe {
        lua_pushnil(l);
        while lua_next(l, index) != 0 {
            let key_type = lua_type(l, -2);

            let key_result = get_table_key(l, key_type);
            if key_result.is_err() {
                lua_pop(l, 1);
                return Err(key_result.unwrap_err());
            }

            let key = key_result.unwrap();

            let value_type = lua_type(l, -1);
            match value_type {
                LUA_TSTRING => {
                    let value_ptr = lua_tostring(l, -1);
                    if value_ptr.is_null() {
                        lua_pop(l, 1);
                        return Err("Invalid value type".to_string());
                    }
                    let value = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
                    doc.insert(key, Bson::String(value));
                }
                LUA_TNUMBER => {
                    let value = lua_tonumber(l, -1);
                    doc.insert(key, Bson::Double(value));
                }
                LUA_TTABLE => {
                    let nested_doc = lua_table_to_bson(l, lua_gettop(l))?;
                    doc.insert(key, Bson::Document(nested_doc));
                }
                _ => {
                    lua_pop(l, 1);
                    return Err("Unsupported value type".to_string());
                }
            }
            lua_pop(l, 1);
        }
    }
    Ok(doc)
}

fn bson_to_lua_table(l: LuaState, doc: Document) {
    #[allow(unused_unsafe)]
    unsafe {
        lua_newtable(l);
        for (key, value) in doc.iter() {
            let key = CString::new(key.to_string()).unwrap();
            lua_pushstring(l, key.as_ptr());
            match value {
                Bson::Double(v) => lua_pushnumber(l, *v),
                Bson::String(v) => {
                    let cstr = CString::new(v.clone()).unwrap();
                    lua_pushstring(l, cstr.as_ptr())
                }
                Bson::Document(v) => {
                    bson_to_lua_table(l, v.clone());
                    lua_settable(l, -3);
                    continue;
                }
                Bson::ObjectId(v) => {
                    let oid_string = v.to_hex();
                    let cstr = CString::new(oid_string).unwrap();
                    lua_pushstring(l, cstr.as_ptr());
                }
                _ => lua_pushnil(l),
            }
            lua_settable(l, -3);
        }
    }
}


// _    _   _   _      _____ _   _ _   _  ____ _____ ___ ___  _   _ ____
// | |  | | | | / \    |  ___| | | | \ | |/ ___|_   _|_ _/ _ \| \ | / ___|
// | |  | | | |/ _ \   | |_  | | | |  \| | |     | |  | | | | |  \| \___ \
// | |__| |_| / ___ \  |  _| | |_| | |\  | |___  | |  | | |_| | |\  |___) |
// |_____\___/_/   \_\ |_|    \___/|_| \_|\____| |_| |___\___/|_| \_|____/

#[lua_function]
pub fn get_collection(l: LuaState) -> i32 {
    let db: Database = read_userdata(l).unwrap();

    let collection_name = rstr!(luaL_checkstring(l, 2));

    let collection: mongodb::Collection<Document> = db.collection(collection_name);

    let collection_list = MONGO_WORKER.block_on(async {
        db.list_collection_names().await
    });

    if collection_list.is_err() {
        log(LogLevel::Error, "Failed to retrieve collection names.");
        return 0;
    }

    if !collection_list.unwrap().contains(&collection_name.to_string()) {
        lua_pushnil(l);
        return 1;
    }

    write_userdata(l, collection);
    luaL_getmetatable(l, cstr!("MongoDBCollection"));
    lua_setmetatable(l, -2);

    1
}

#[lua_function]
pub fn drop_collection(l: LuaState) -> i32 {
    let db: Database = read_userdata(l).unwrap();

    let collection_name = rstr!(luaL_checkstring(l, 2));

    let collection_list = MONGO_WORKER.block_on(async {
        db.list_collection_names().await
    });

    if collection_list.is_err() {
        log(LogLevel::Error, "Failed to retrieve collection names.");
        lua_pushboolean(l, false as i32);
        return 1;
    }

    if !collection_list.unwrap().contains(&collection_name.to_string()) {
        log(LogLevel::Warning, &format!("Trying to drop collection '{}', but it doesn't exist in '{}'.", collection_name, db.name()));
        lua_pushboolean(l, false as i32);
        return 1;
    }

    MONGO_WORKER.block_on(async {
        let _ = db.collection::<Document>(collection_name).drop().await;
    });

    lua_pushboolean(l, true as i32);

    1
}

#[lua_function]
pub fn create_collection(_l: LuaState) -> i32 {
    let db: Database = read_userdata(_l).unwrap();
    let collection_name = rstr!(luaL_checkstring(_l, 2));

    let collection_list = MONGO_WORKER.block_on(async {
        db.list_collection_names().await
    });

    if collection_list.is_err() {
        log(LogLevel::Error, "Failed to retrieve collection names.");
        lua_pushboolean(_l, 0); // Error
        return 0;
    }

    if collection_list.unwrap().contains(&collection_name.to_string()) {
        lua_pushboolean(_l, 0); // Error
        return 1;
    }

    let result = MONGO_WORKER.block_on(async {
        db.create_collection(collection_name).await
    });

    match result {
        Ok(_) => lua_pushboolean(_l, 1),
        Err(_) => {
            log(LogLevel::Error, &format!("Failed to create collection: '{}.'", collection_name));
            lua_pushboolean(_l, 0);
        }
    }

    1
}

#[lua_function]
pub fn insert(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l) {
        Ok(col) => col,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to get collection: {}", err));
            lua_pushboolean(l, false as i32);
            return 1;
        }
    };

    let doc = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to convert table to BSON: {}", err));
            lua_pushboolean(l, false as i32);
            return 1;
        }
    };

    let insert_result = MONGO_WORKER.block_on(async {
        collection.insert_one(doc).await
    });

    if let Err(err) = insert_result {
        log(LogLevel::Error, &format!("Failed to insert document: {}", err));
        lua_pushboolean(l, false as i32);
        return 1;
    }

    lua_pushboolean(l, true as i32);
    1
}

#[lua_function]
pub fn find(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l) {
        Ok(col) => col,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to get collection: {}", err));
            lua_pushnil(l);
            return 1;
        }
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to convert table to BSON: {}", err));
            lua_pushnil(l);
            return 1;
        }
    };

    let find_result = MONGO_WORKER.block_on(async {
        collection.find(filter).await
    });

    if let Err(err) = find_result {
        log(LogLevel::Error, &format!("Failed to execute find: {}", err));
        lua_pushnil(l);
        return 1;
    }

    let cursor = find_result.unwrap();
    let docs: Vec<Document> = MONGO_WORKER.block_on(async {
        cursor.try_collect().await.unwrap_or_else(|_| Vec::new())
    });

    lua_newtable(l);
    for (i, doc) in docs.iter().enumerate() {
        bson_to_lua_table(l, doc.clone());
        lua_rawseti(l, -2, i as i32 + 1);
    }

    1
}

#[lua_function]
pub fn update(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l) {
        Ok(col) => col,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to get collection: {}", err));
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to convert filter to BSON: {}", err));
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let update = match lua_table_to_bson(l, 3) {
        Ok(doc) => doc,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to convert update to BSON: {}", err));
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let update_result = MONGO_WORKER.block_on(async {
        collection.update_many(filter, update).await
    });

    match update_result {
        Ok(_) => lua_pushboolean(l, 1),
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to execute update: {}", err));
            lua_pushboolean(l, 0);
        }
    }

    1
}

#[lua_function]
pub fn delete(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l) {
        Ok(col) => col,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to get collection: {}", err));
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to convert filter to BSON: {}", err));
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let delete_result = MONGO_WORKER.block_on(async {
        collection.delete_many(filter).await
    });

    match delete_result {
        Ok(_) => lua_pushboolean(l, 1),
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to execute delete: {}", err));
            lua_pushboolean(l, 0);
        }
    }

    1
}
