#![allow(dead_code)]

use mongodb::{Database};
use mongodb::bson::Document;
use rglua::lua::{lua_getmetatable, lua_pushboolean, lua_pushlightuserdata, lua_setmetatable, lua_touserdata, luaL_checkstring, luaL_getmetatable, LuaState};
use crate::logger::{log, LogLevel};
use crate::mongo::MONGO_WORKER;

fn send_collection(l: LuaState, collection: mongodb::Collection<Document>) {
    let collection_ptr = Box::into_raw(Box::new(collection));
    lua_pushlightuserdata(l, collection_ptr as *mut std::ffi::c_void);
}

fn get_database(l: LuaState) -> Result<Database, String> {
    unsafe {
        let database_ptr = lua_touserdata(l, 1) as *mut Database;
        if database_ptr.is_null() {
            return Err("Database is null".to_string());
        }

        let database = Box::from_raw(database_ptr);
        Ok(*database)
    }
}

fn get_current_collection(l: LuaState) -> Result<mongodb::Collection<Document>, String> {
    unsafe {
        let collection_ptr = lua_touserdata(l, 1) as *mut mongodb::Collection<Document>;
        if collection_ptr.is_null() {
            return Err("Collection is null".to_string());
        }

        let collection = Box::from_raw(collection_ptr);
        Ok(*collection)
    }
}

#[lua_function]
pub fn get_collection(l: LuaState) -> i32 {
    let db = get_database(l).unwrap();

    let collection_name = rstr!(luaL_checkstring(l, 2));
    log(LogLevel::Debug, &format!("Retrieving collection '{}'...", collection_name));
    log(LogLevel::Debug, &format!("Looking on '{}'", db.name()));

    // Check if collection exists

    let collection: mongodb::Collection<Document> = db.collection(collection_name);


    let collection_list = MONGO_WORKER.block_on(async {
        db.list_collection_names(None).await
    });

    if collection_list.is_err() {
        log(LogLevel::Error, "Failed to retrieve collection names.");
        return 0;
    }

    if !collection_list.unwrap().contains(&collection_name.to_string()) {
        log(LogLevel::Warning, &format!("Trying to get collection '{}', but it doesn't exist in '{}'.", collection_name, db.name()));
        return 0;
    }

    send_collection(l, collection);
    luaL_getmetatable(l, cstr!("MongoDBCollection"));
    lua_setmetatable(l, -2);

    1
}

#[lua_function]
pub fn drop_collection(l: LuaState) -> i32 {
    let db = get_database(l).unwrap();

    let collection_name = rstr!(luaL_checkstring(l, 2));

    log(LogLevel::Debug, &format!("Dropping collection '{}'...", collection_name));
    log(LogLevel::Debug, &format!("Looking on '{}'", db.name()));

    let collection_list = MONGO_WORKER.block_on(async {
        db.list_collection_names(None).await
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
        db.collection::<Document>(collection_name).drop(None).await.expect("Failed to drop collection");
        lua_pushboolean(l, true as i32);
        return 1;
    });

    1
}

#[lua_function]
pub fn create_collection(_l: LuaState) -> i32 {
    let db = get_database(_l).unwrap();
    let collection_name = rstr!(luaL_checkstring(_l, 2));

    let collection_list = MONGO_WORKER.block_on(async {
        db.list_collection_names(None).await
    });

    if collection_list.is_err() {
        log(LogLevel::Error, "Failed to retrieve collection names.");
        return 0;
    }

    if collection_list.unwrap().contains(&collection_name.to_string()) {
        log(LogLevel::Warning, &format!("Trying to create collection '{}', but it already exists in '{}'.", collection_name, db.name()));
        return 0;
    }

    log(LogLevel::Debug, &format!("Creating collection '{}'...", collection_name));

    let result = MONGO_WORKER.block_on(async {
        db.create_collection(collection_name, None).await
    });

    1
}

#[lua_function]
pub unsafe fn insert(_l: LuaState) -> i32 {
    return 1;
}

#[lua_function]
pub unsafe fn find(_l: LuaState) -> i32 {
    return 1;
}

#[lua_function]
pub unsafe fn update(_l: LuaState) -> i32 {
    return 1;
}

#[lua_function]
pub unsafe fn delete(_l: LuaState) -> i32 {
    return 1;
}
