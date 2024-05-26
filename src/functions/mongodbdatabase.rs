#![allow(dead_code)]

use mongodb::Client;
use rglua::lua::LuaState;
use rglua::prelude::{lua_pushlightuserdata, lua_setmetatable, lua_touserdata, luaL_checkstring, luaL_getmetatable};
use tokio::runtime::Runtime;

use crate::logger::{log, LogLevel};

fn get_client(l: LuaState) -> Result<Client, String> {
    unsafe {
        let client_ptr = lua_touserdata(l, 1) as *mut Client;
        if client_ptr.is_null() {
            return Err("Client is null".to_string());
        }

        let client = Box::from_raw(client_ptr);
        Ok(*client)
    }
}

#[lua_function]
pub fn get_database(l: LuaState) -> i32 {
    let rt = Runtime::new().unwrap();
    let client = get_client(l).unwrap();

    let database_name = rstr!(luaL_checkstring(l, 2));
    log(LogLevel::Info, &format!("Retrieving database '{}'...", database_name));

    let db = client.database(database_name);

    unsafe {
        let db_ptr = Box::into_raw(Box::new(db));
        lua_pushlightuserdata(l, db_ptr as *mut std::ffi::c_void);
    }

    luaL_getmetatable(l, cstr!("MongoDBDatabase"));
    lua_setmetatable(l, -2);

    1
}

