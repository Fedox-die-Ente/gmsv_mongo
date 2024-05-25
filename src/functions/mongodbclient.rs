use mongodb::Client;
use rglua::lua::{lua_newuserdata, LuaState};
use rglua::prelude::{lua_setmetatable, lua_touserdata, luaL_checkstring, luaL_getmetatable};

use crate::logger::{log, LogLevel};
use crate::mongo::connect_to_db;

#[lua_function]
pub fn new_client(l: LuaState) -> i32 {
    let connection_url = rstr!(luaL_checkstring(l, 1));
    log(LogLevel::Info, &format!("Connecting to MongoDB at {:?}", connection_url));

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = runtime.block_on(connect_to_db(&*connection_url)).unwrap();

    unsafe {
        let client_ptr = lua_newuserdata(l, std::mem::size_of::<Client>());
        let client_ptr = client_ptr as *mut Client;
        std::ptr::write(client_ptr, client);
        luaL_getmetatable(l, cstr!("MongoDBClient"));
        lua_setmetatable(l, -2);
    }

    return 1;
}

#[lua_function]
pub fn get_database(_l: LuaState) -> i32 {
    let client = unsafe {
        let client_ptr = lua_touserdata(_l, 1) as *mut Client;
        &*client_ptr
    };

    let database_name = rstr!(luaL_checkstring(_l, 2));
    let db = client.database(&*database_name);
    log(LogLevel::Info, &format!("Retrieved database {:?}", db.name()));

    unsafe {
        let db_ptr = lua_newuserdata(_l, std::mem::size_of::<mongodb::Database>());
        let db_ptr = db_ptr as *mut mongodb::Database;
        std::ptr::write(db_ptr, db);
        luaL_getmetatable(_l, cstr!("MongoDBDatabase"));
        lua_setmetatable(_l, -2);
    }

    return 1;
}
