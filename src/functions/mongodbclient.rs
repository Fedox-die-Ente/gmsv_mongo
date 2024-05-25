use gmod::lua::LUA_REGISTRYINDEX;

use crate::logger::{log, LogLevel};

#[lua_function]
pub unsafe fn new_client(lua: gmod::lua::State) -> i32 {
    lua.get_field(1, lua_string!("connection_url"));
    let connection_url = lua.get_string(1).unwrap();
    log(LogLevel::Debug, &*format!("{}", connection_url));

    let metatable = lua.get_field(LUA_REGISTRYINDEX, lua_string!("MongoDBClient"));

    return 1;
}

#[lua_function]
pub unsafe fn get_database(lua: gmod::lua::State) -> i32 {
    lua.get_field(1, lua_string!("database_name"));
    let database_name = lua.get_string(1).unwrap();
    log(
        LogLevel::Debug,
        &*format!("Database Name: {}", database_name),
    );
    return 1;
}
