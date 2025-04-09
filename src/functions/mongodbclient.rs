use rglua::lua::LuaState;
use rglua::prelude::{lua_setmetatable, luaL_checkstring, luaL_error, luaL_getmetatable, lua_pushnil};
use mongodb::bson::doc;

use crate::logger::{log, LogLevel};
use crate::mongo::{create_client_options, create_mongo_client, MONGO_WORKER};
use crate::utils::luautils::write_userdata;

#[lua_function]
pub fn new_client(l: LuaState) -> i32 {
    let connection_url = rstr!(luaL_checkstring(l, 1));

    if !connection_url.starts_with("mongodb://") && !connection_url.starts_with("mongodb+srv://") {
        luaL_error(l, cstr!("Invalid connection URL. Must start with 'mongodb://' or 'mongodb+srv://'."));
    }

    let client_options = match create_client_options(connection_url.to_string()) {
        Ok(opts) => opts,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to create client options: {}", err));
            lua_pushnil(l);
            return 1;
        }
    };

    let client = match create_mongo_client(client_options) {
        Ok(client) => client,
        Err(err) => {
            log(LogLevel::Error, &format!("Failed to connect to MongoDB: {}", err));
            lua_pushnil(l);
            return 1;
        }
    };

    let ping_result = MONGO_WORKER.block_on(async {
        client
            .database("admin")
            .run_command(doc! {"ping": 1})
            .await
    });

    if ping_result.is_err() {
        log(
            LogLevel::Error,
            &format!("Connection test failed: {}", ping_result.unwrap_err()),
        );
        lua_pushnil(l);
        return 1;
    }

    log(LogLevel::Info, "Successfully connected to MongoDB.");
    write_userdata(l, client);
    luaL_getmetatable(l, cstr!("MongoDBClient"));
    lua_setmetatable(l, -2);

    1
}