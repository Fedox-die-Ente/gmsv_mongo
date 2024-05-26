use mongodb::Client;
use rglua::lua::LuaState;
use rglua::prelude::{lua_call, lua_pushlightuserdata, lua_setmetatable, lua_touserdata, luaL_checkstring, luaL_getmetatable};
use tokio::runtime::Runtime;

use crate::logger::{log, LogLevel};
use crate::mongo::{craete_client_options, create_mongo_client};

fn get_client(l: LuaState) -> Result<Client, String> {
    unsafe {
        let client_ptr = lua_touserdata(l, 1) as *mut Client;
        if client_ptr.is_null() {
            return Err("Client is null".to_string());
        }

        let client = Box::from_raw(client_ptr);
        log(LogLevel::Info, "Client retrieved.");
        Ok(*client)
    }
}

#[lua_function]
pub fn new_client(l: LuaState) -> i32 {
    let connection_url = rstr!(luaL_checkstring(l, 1));
    log(LogLevel::Info, &format!("Connecting to MongoDB at {:?}", connection_url));

    let receiver = craete_client_options(connection_url.to_string());

    match receiver.recv().unwrap() {
        Ok(client_options) => {
            let client = create_mongo_client(client_options);
            log(LogLevel::Info, "Connected to MongoDB");

            unsafe {
                let client_ptr = Box::into_raw(Box::new(client));
                lua_pushlightuserdata(l, client_ptr as *mut std::ffi::c_void);
            }

            luaL_getmetatable(l, cstr!("MongoDBClient"));
            lua_setmetatable(l, -2);
            lua_call(l, 1, 0);
        }
        Err(e) => {
            log(LogLevel::Error, &format!("Failed to connect to MongoDB: {}", e));
        }
    }

    0
}

#[lua_function]
pub fn get_database(l: LuaState) -> i32 {
    let rt = Runtime::new().unwrap();
    let client = get_client(l).unwrap();

    let database_name = rstr!(luaL_checkstring(l, 2));
    log(LogLevel::Info, &format!("Retrieving database '{}'...", database_name));

    log(LogLevel::Debug, &format!("Client: {:?}", client));
    let db = client.database("admin");
    log(LogLevel::Debug, &format!("Database: {:?}", db));
    return 1;
}

