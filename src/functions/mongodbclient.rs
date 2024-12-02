use rglua::lua::LuaState;
use rglua::prelude::{lua_setmetatable, luaL_checkstring, luaL_error, luaL_getmetatable};

use crate::logger::{log, LogLevel};
use crate::mongo::{create_client_options, create_mongo_client};
use crate::utils::luautils::write_userdata;

#[lua_function]
pub fn new_client(l: LuaState) -> i32 {
    let connection_url = rstr!(luaL_checkstring(l, 1));

    if !connection_url.starts_with("mongodb://") && !connection_url.starts_with("mongodb+srv://") {
        luaL_error(l, cstr!("Invalid connection URL. Must start with 'mongodb://' or 'mongodb+srv://'."));
        return 0;
    }

    let client_options = create_client_options(connection_url.to_string());
    let client = create_mongo_client(client_options);
    log(LogLevel::Info, "Successfully connected to MongoDB.");

    write_userdata(l, client);
    luaL_getmetatable(l, cstr!("MongoDBClient"));
    lua_setmetatable(l, -2);

    1
}