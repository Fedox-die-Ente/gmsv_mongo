#![allow(dead_code)]

use crate::logger::{log, LogLevel};

#[lua_function]
pub unsafe fn get_collection(lua: gmod::lua::State) -> i32 {
    lua.get_field(1, lua_string!("collection_name"));
    let collection_name = lua.get_string(1).unwrap();
    log(LogLevel::Debug, &*format!("Collection Name: {}", collection_name));

    return 1;
}