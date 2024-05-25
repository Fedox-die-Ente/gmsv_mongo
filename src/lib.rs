#![feature(c_unwind)]
#![allow(dead_code)]
extern crate core;
#[macro_use]
extern crate gmod;

use std::collections::HashMap;
use std::sync::Mutex;

use gmod::lua::LuaNumber;
use lazy_static::lazy_static;

use logger::log;
use logger::LogLevel;

use crate::functions::mongodbclient::new_client;

mod logger;
mod mongo;
mod functions;

mod tests;

lazy_static! {
    static ref MONGO_METATABLES: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());

    static ref IDENT_MONGODBCLIENT: String = "MongoDBClient".to_string();
    static ref IDENT_MONGODBDATABASE: String = "MongoDBDatabase".to_string();
    static ref IDENT_MONGODBCOLLECTION: String = "MongoDBCollection".to_string();
}

pub fn get_metatable_index(metatable: String) -> i32 {
    let metatables = MONGO_METATABLES.lock().unwrap();
    return metatables.get(&metatable).unwrap().clone();
}

#[lua_function]
unsafe fn hello_world(lua: gmod::lua::State) -> i32 {
    lua.get_global(lua_string!("print"));
    lua.push_string("Hello, world!");
    lua.call(1, 0);
    return 0;
}

#[lua_function]
unsafe fn mongo_dbclient(lua: gmod::lua::State) -> i32 {
    let connection_string = match lua.get_string(1) {
        Some(string) => string,
        None => {
            eprintln!("Error: Connection string not provided");
            return -1;
        }
    };


    let msg = format!("Connection string: {}", connection_string);
    log(LogLevel::Debug, &*msg);

    return 1;
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
    let cargo_name = env!("CARGO_PKG_NAME");
    let cargo_version = env!("CARGO_PKG_VERSION");
    let log_message = format!("Module '{} ({})' loaded and ready.", cargo_name, cargo_version);

    log(LogLevel::Info, &*log_message);

    lua.push_function(hello_world);
    lua.set_global(lua_string!("hello_world"));

    let mut metatables = MONGO_METATABLES.lock().unwrap();

    lua.new_metatable(lua_string!("MongoDBClient"));
    metatables.insert(IDENT_MONGODBCLIENT.clone(), lua.get_top());
    lua.push_number(-1 as LuaNumber);
    lua.set_field(-2, lua_string!("__index"));
    lua.pop();

    lua.new_metatable(lua_string!("MongoDBDatabase"));
    metatables.insert(IDENT_MONGODBDATABASE.clone(), lua.get_top());
    lua.push_number(-1 as LuaNumber);
    lua.set_field(-2, lua_string!("__index"));
    lua.pop();

    lua.new_metatable(lua_string!("MongoDBCollection"));
    metatables.insert(IDENT_MONGODBCOLLECTION.clone(), lua.get_top());
    lua.push_number(-1 as LuaNumber);
    lua.set_field(-2, lua_string!("__index"));
    lua.pop();

    lua.new_table();
    lua.push_function(new_client);
    lua.set_field(-2, lua_string!("Client"));
    lua.set_global(lua_string!("MongoDB"));

    log(LogLevel::Debug, &*format!("Registered meta tables: {:?}", metatables));

    return 0;
}


#[gmod13_close]
fn gmod13_close(_lua: gmod::lua::State) -> i32 {
    println!("Goodbye from binary module!");

    let cargo_name = env!("CARGO_PKG_NAME");
    let cargo_version = env!("CARGO_PKG_VERSION");
    let log_message = format!("Module '{} ({})' is dying now.", cargo_name, cargo_version);

    log(LogLevel::Info, &*log_message);

    return 0;
}

