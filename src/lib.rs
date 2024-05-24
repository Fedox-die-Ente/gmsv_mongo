#![feature(c_unwind)]

#[macro_use]
extern crate gmod;

use logger::log;
use logger::LogLevel;

mod logger;

#[gmod13_open]
fn gmod13_open(lua: gmod::lua::State) -> i32 {
    log(LogLevel::Info, "GMSV_MONGO Binary loaded and ready.").expect("Failed to log.");
    0
}

#[gmod13_close]
fn gmod13_close(_lua: gmod::lua::State) -> i32 {
    println!("Goodbye from binary module!");
    log(LogLevel::Info, "GMSV_MONGO Binary shutting down.").expect("Failed to log.");
    0
}