#![allow(dead_code)]

use rglua::lua::LuaState;

#[lua_function]
pub unsafe fn get_collection(_l: LuaState) -> i32 {
    1
}