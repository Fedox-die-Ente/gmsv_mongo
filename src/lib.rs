extern crate core;
#[macro_use]
extern crate rglua;

use rglua::lua::LuaState;
use rglua::prelude::*;

use logger::log;
use logger::LogLevel;

use crate::functions::mongodbclient::new_client;
use crate::functions::mongodbcollection::{create_collection, delete, drop_collection, find, get_collection, insert, update};
use crate::functions::mongodbdatabase::get_database;
use crate::updatecheck::check_latest_version;
use std::sync::atomic::{AtomicBool, Ordering};

mod logger;
mod mongo;
mod functions;
mod tests;
mod utils;
mod updatecheck;

static SUPPRESS_MESSAGES: AtomicBool = AtomicBool::new(false);

#[gmod_open]
unsafe fn open(l: LuaState) -> i32 {
    let cargo_name = env!("CARGO_PKG_NAME");
    let cargo_version = env!("CARGO_PKG_VERSION");
    let log_message = format!("Module '{} ({})' loaded and ready.", cargo_name, cargo_version);
    log(LogLevel::Info, &*log_message);

    check_latest_version().unwrap();

    luaL_newmetatable(l, cstr!("MongoDBClient"));
    lua_pushvalue(l, -1);
    lua_setfield(l, -2, cstr!("__index"));
    lua_pushcfunction(l, get_database);
    lua_setfield(l, -2, cstr!("Database"));

    luaL_newmetatable(l, cstr!("MongoDBDatabase"));
    lua_pushvalue(l, -1);
    lua_setfield(l, -2, cstr!("__index"));
    lua_pushcfunction(l, get_collection);
    lua_setfield(l, -2, cstr!("Collection"));
    lua_pushcfunction(l, drop_collection);
    lua_setfield(l, -2, cstr!("DropCollection"));
    lua_pushcfunction(l, create_collection);
    lua_setfield(l, -2, cstr!("CreateCollection"));

    luaL_newmetatable(l, cstr!("MongoDBCollection"));
    lua_pushvalue(l, -1);
    lua_setfield(l, -2, cstr!("__index"));
    lua_pushcfunction(l, insert);
    lua_setfield(l, -2, cstr!("Insert"));
    lua_pushcfunction(l, find);
    lua_setfield(l, -2, cstr!("Find"));
    lua_pushcfunction(l, update);
    lua_setfield(l, -2, cstr!("Update"));
    lua_pushcfunction(l, delete);
    lua_setfield(l, -2, cstr!("Delete"));

    lua_newtable(l);
    lua_pushcfunction(l, new_client);
    lua_setfield(l, -2, cstr!("Client"));

    lua_pushcfunction(l, suppress_messages);
    lua_setfield(l, -2, cstr!("SuppressMessages"));

    lua_setglobal(l, cstr!("MongoDB"));

    return 0;
}

extern "C" fn suppress_messages(l: LuaState) -> i32 {
    let suppress = lua_toboolean(l, 1) != 0;
    SUPPRESS_MESSAGES.store(suppress, Ordering::Relaxed);
    0
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
    let cargo_name = env!("CARGO_PKG_NAME");
    let cargo_version = env!("CARGO_PKG_VERSION");
    let log_message = format!("Module '{} ({})' is dying now.", cargo_name, cargo_version);

    log(LogLevel::Info, &*log_message);

    return 0;
}

