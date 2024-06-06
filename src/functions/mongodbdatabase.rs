use mongodb::Client;
use rglua::lua::LuaState;
use rglua::prelude::{lua_setmetatable, luaL_checkstring, luaL_getmetatable};

use crate::utils::luautils::{read_userdata, write_userdata};

#[lua_function]
pub fn get_database(l: LuaState) -> i32 {
    let client: Client = read_userdata(l).unwrap();

    let database_name = rstr!(luaL_checkstring(l, 2));
    let db = client.database(database_name);

    write_userdata(l, db);
    luaL_getmetatable(l, cstr!("MongoDBDatabase"));
    lua_setmetatable(l, -2);

    1
}
