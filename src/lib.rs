#![feature(c_unwind)]

extern crate core;
#[macro_use]
extern crate gmod;

use gmod::lua::LuaNumber;

use logger::log;
use logger::LogLevel;

use crate::functions::mongodbclient::connect;

mod logger;
mod mongo;
mod functions;
mod tests;

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

    lua.new_metatable(lua_string!("MongoDBClient"));
    lua.push_number(-1 as LuaNumber);
    lua.set_field(-2, lua_string!("__index"));
    lua.push_function(connect);
    lua.set_field(-2, lua_string!("Connect"));
    lua.pop();

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

