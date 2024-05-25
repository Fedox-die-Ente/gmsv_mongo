use rglua::lua::{luaL_checkstring, lua_tostring, LuaState};

use crate::logger::{log, LogLevel};

#[lua_function]
pub fn new_client(l: LuaState) -> i32 {
    let connection_url = rstr!(luaL_checkstring(l, 1));
    log(LogLevel::Info, &format!("Connecting to MongoDB at {:?}", connection_url));
    return 1;
}

#[lua_function]
pub fn get_database(_l: LuaState) -> i32 {
    
    return 1;
}
