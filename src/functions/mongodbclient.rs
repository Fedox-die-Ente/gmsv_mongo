use std::os::raw::c_void;

use rglua::lua::{lua_newuserdata, lua_setfield, LuaState, LuaType, Userdata};
use rglua::prelude::{lua_pushcfunction, lua_setmetatable, luaL_checkstring, luaL_checkudata, luaL_getmetatable};

use crate::logger::{log, LogLevel};
use crate::mongo::connect_to_db;

#[lua_function]
pub fn new_client(l: LuaState) -> i32 {
    let connection_url = rstr!(luaL_checkstring(l, 1));
    log(LogLevel::Info, &format!("Connecting to MongoDB at {:?}", connection_url));

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = runtime.block_on(connect_to_db(&*connection_url)).unwrap();

    let ptr = lua_newuserdata(l, std::mem::size_of::<Userdata>());

    unsafe {
        let ty = std::ptr::addr_of_mut!((*ptr).typ);
        ty.write(LuaType::Userdata);

        let data = std::ptr::addr_of_mut!((*ptr).data);
        data.write(Box::into_raw(Box::new(client)) as *mut c_void);
    }

    lua_pushcfunction(l, get_database);
    lua_setfield(l, -2, cstr!("Database"));

    luaL_getmetatable(l, cstr!("MongoDBClient"));
    lua_setmetatable(l, -2);

    return 1;
}

#[lua_function]
pub fn get_database(_l: LuaState) -> i32 {
    let client = unsafe {
        let client = luaL_checkudata(_l, 1, cstr!("MongoDBClient")) as *mut Userdata;
        let client = (*client).data as *mut mongodb::Client;
        &*client
    };

    let database_name = rstr!(luaL_checkstring(_l, 2));
    let db = client.database(&*database_name);

    let ptr = lua_newuserdata(_l, std::mem::size_of::<Userdata>());

    unsafe {
        let ty = std::ptr::addr_of_mut!((*ptr).typ);
        ty.write(LuaType::Userdata);

        let data = std::ptr::addr_of_mut!((*ptr).data);
        data.write(Box::into_raw(Box::new(db)) as *mut c_void);
    }

    luaL_getmetatable(_l, cstr!("MongoDBDatabase"));
    lua_setmetatable(_l, -2);

    return 1;
}
