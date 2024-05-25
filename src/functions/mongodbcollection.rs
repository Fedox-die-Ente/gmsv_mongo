#![allow(dead_code)]

use rglua::lua::LuaState;

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