#![allow(dead_code)]

use rglua::lua::{luaL_checkstring, LuaState};

use crate::logger::{log, LogLevel};

#[lua_function]
pub fn get_collection(l: LuaState) -> i32 {
    let collection_name = rstr!(luaL_checkstring(l, 2));
    log(LogLevel::Debug, &format!("Retrieving collection '{}'...", collection_name));

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
