use mongodb::{Client, Database};
use rglua::lua::{lua_newuserdata, LuaState};
use rglua::prelude::{lua_setmetatable, lua_touserdata, luaL_checkstring, luaL_getmetatable};

use crate::logger::{log, LogLevel};

fn get_client(l: LuaState) -> Option<Client> {
    let client_ptr = lua_touserdata(l, 1) as *mut Client;
    if client_ptr.is_null() {
        None
    } else {
        Some(unsafe { (*client_ptr).clone() })
    }
}

fn send_database(l: LuaState, db: mongodb::Database) {
    let db_ptr = lua_newuserdata(l, std::mem::size_of::<Client>()) as *mut Database;
    unsafe {
        std::ptr::write(db_ptr, db);
    }
}

#[lua_function]
pub fn get_database(l: LuaState) -> i32 {
    let client = get_client(l).unwrap();

    let database_name = rstr!(luaL_checkstring(l, 2));
    let db = client.database(database_name);

    send_database(l, db);
    luaL_getmetatable(l, cstr!("MongoDBDatabase"));
    lua_setmetatable(l, -2);

    1
}
