#![allow(dead_code)]

use mongodb::bson::Document;
use crate::logger::{log, LogLevel};

#[lua_function]
pub unsafe fn insert(lua: gmod::lua::State) -> i32 {
    return 1;
}

#[lua_function]
pub unsafe fn find(lua: gmod::lua::State) -> i32 {
    return 1;
}

#[lua_function]
pub unsafe fn update(lua: gmod::lua::State) -> i32 {
    return 1;
}

#[lua_function]
pub unsafe fn delete(lua: gmod::lua::State) -> i32 {
    return 1;
}