#![feature(c_unwind)]

#[macro_use]
extern crate gmod;

use logger::log;
use logger::LogLevel;

mod logger;
mod mongo;

#[lua_function]
unsafe fn hello_world(lua: gmod::lua::State) -> i32 {
    lua.get_global(lua_string!("print"));
    lua.push_string("Hello, world!");
    lua.call(1, 0);
    return 0;
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
    log(LogLevel::Info, "GMSV_MONGO Binary loaded and ready.")
        .expect("Failed to log.");

    lua.push_function(hello_world);
    lua.set_global(lua_string!("hello_world"));

    return 0;
}


#[gmod13_close]
fn gmod13_close(_lua: gmod::lua::State) -> i32 {
    println!("Goodbye from binary module!");

    log(LogLevel::Info, "GMSV_MONGO Binary shutting down.")
        .expect("Failed to log.");
    return 0;
}

