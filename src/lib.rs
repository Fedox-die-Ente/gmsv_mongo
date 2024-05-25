#![feature(c_unwind)]
#![allow(dead_code)]
extern crate core;
#[macro_use]
extern crate rglua;

use rglua::lua::LuaState;
use rglua::prelude::*;

use logger::log;
use logger::LogLevel;

use crate::functions::mongodbclient::{get_database, new_client};

mod logger;
mod mongo;
mod functions;
mod tests;

#[gmod_open]
unsafe fn open(l: LuaState) -> i32 {
    let cargo_name = env!("CARGO_PKG_NAME");
    let cargo_version = env!("CARGO_PKG_VERSION");
    let log_message = format!("Module '{} ({})' loaded and ready.", cargo_name, cargo_version);

    log(LogLevel::Info, &*log_message);

    luaL_newmetatable(l, cstr!("MongoDBClient"));
    lua_pushvalue(l, -1);
    lua_setfield(l, -2, cstr!("__index"));
    lua_pushcfunction(l, get_database);
    lua_setfield(l, -2, cstr!("Database"));

    lua_newtable(l);
    lua_pushcfunction(l, new_client);
    lua_setfield(l, -2, cstr!("Client"));
    lua_setglobal(l, cstr!("MongoDB"));

    return 0;
}


#[gmod_close]
fn close(_l: LuaState) -> i32 {
    println!("Goodbye from binary module!");

    let cargo_name = env!("CARGO_PKG_NAME");
    let cargo_version = env!("CARGO_PKG_VERSION");
    let log_message = format!("Module '{} ({})' is dying now.", cargo_name, cargo_version);

    log(LogLevel::Info, &*log_message);

    return 0;
}

