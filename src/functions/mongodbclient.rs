use mongodb::Client;
use rglua::lua::LuaState;
use rglua::prelude::{lua_pushlightuserdata, lua_setmetatable, luaL_checkstring, luaL_getmetatable};

use crate::logger::{log, LogLevel};
use crate::mongo::{create_client_options, create_mongo_client};

fn send_client(l: LuaState, client: Client) {
    unsafe {
        let client_ptr = Box::into_raw(Box::new(client));
        lua_pushlightuserdata(l, client_ptr as *mut std::ffi::c_void);
    }
}

#[lua_function]
pub fn new_client(l: LuaState) -> i32 {
    let connection_url = rstr!(luaL_checkstring(l, 1));
    log(LogLevel::Info, &format!("Connecting to MongoDB at {:?}", connection_url));

    let client_options = create_client_options(connection_url.to_string());
    let client = create_mongo_client(client_options);
    log(LogLevel::Info, "Connected to MongoDB");

    send_client(l, client);
    luaL_getmetatable(l, cstr!("MongoDBClient"));
    lua_setmetatable(l, -2);

    1
}