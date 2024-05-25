use gmod::lua::LUA_REGISTRYINDEX;
use mongodb::Client;

use crate::logger::{log, LogLevel};
use crate::mongo::connect_to_db;

#[lua_function]
pub unsafe fn new_client(lua: gmod::lua::State) -> i32 {
    lua.get_field(1, lua_string!("connection_url"));
    let connection_url = lua.get_string(1).unwrap();
    log(LogLevel::Debug, &*format!("{}", connection_url));

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = runtime.block_on(connect_to_db(&*connection_url)).unwrap();

    lua.get_field(LUA_REGISTRYINDEX, lua_string!("MongoDBClient"));
    let metatable = lua.get_metatable(-1);
    lua.new_userdata(client, Option::from(metatable));

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

    lua.get_field(LUA_REGISTRYINDEX, lua_string!("MongoDBClient"));
    let client = lua.get_userdata::<Client>(-1).unwrap();
    let db = client.database(database_name);
    log(LogLevel::Debug, &*format!("Database from client: {:?}", db));

    lua.get_field(LUA_REGISTRYINDEX, lua_string!("MongoDBDatabase"));
    let metatable = lua.get_metatable(-1);
    lua.new_userdata(db, Option::from(metatable));

    return 1;
}
