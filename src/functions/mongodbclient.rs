use mongodb::Client;
use rglua::lua::{lua_newuserdata, LuaState};
use rglua::prelude::{lua_setmetatable, luaL_checkstring, luaL_getmetatable};

use crate::logger::{log, LogLevel};
use crate::mongo::connect_to_db;

#[lua_function]
pub fn new_client(l: LuaState) -> i32 {
    let connection_url = rstr!(luaL_checkstring(l, 1));
    log(LogLevel::Info, &format!("Connecting to MongoDB at {:?}", connection_url));

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = runtime.block_on(connect_to_db(&*connection_url)).unwrap();

    unsafe {
        // create new userdata and store it in MongoDBClient Metatable
        let client_ptr = lua_newuserdata(l, std::mem::size_of::<Client>());
        let client_ptr = client_ptr as *mut Client;
        std::ptr::write(client_ptr, client);
        luaL_getmetatable(l, cstr!("MongoDBClient"));
        lua_setmetatable(l, -2);
    }

    return 1;
}

#[lua_function]
pub fn get_database(_l: LuaState) -> i32 {
    return 1;
}
