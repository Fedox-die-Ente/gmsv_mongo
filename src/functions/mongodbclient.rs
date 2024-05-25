use std::os::raw::c_void;

use mongodb::Client;
use rglua::lua::{lua_newuserdata, LuaState};
use rglua::prelude::{lua_setmetatable, luaL_checkstring, luaL_checkudata, luaL_getmetatable};

use crate::logger::{log, LogLevel};
use crate::mongo::connect_to_db;

#[lua_function]
pub fn new_client(l: LuaState) -> i32 {
    let connection_url = rstr!(luaL_checkstring(l, 1));
    log(LogLevel::Info, &format!("Connecting to MongoDB at {:?}", connection_url));

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = runtime.block_on(connect_to_db(&*connection_url)).unwrap();

    let ptr = lua_newuserdata(l, std::mem::size_of::<Client>());

    unsafe {
        let data = std::ptr::addr_of_mut!((*ptr).data);
        data.write(Box::into_raw(Box::new(client)) as *mut c_void);
    }

    luaL_getmetatable(l, cstr!("MongoDBClient"));
    lua_setmetatable(l, -2);

    return 1;
}

#[lua_function]
pub fn get_database(l: LuaState) -> i32 {
    let client = unsafe {
        let ptr = luaL_checkudata(l, 1, cstr!("MongoDBClient")) as *mut Client;
        Box::from_raw(ptr)
    };

    let database_name = rstr!(luaL_checkstring(l, 2));
    log(LogLevel::Info, &format!("Retrieved database '{}'", database_name));

    return 1;
}

