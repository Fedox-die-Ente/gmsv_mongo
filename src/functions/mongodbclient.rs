use std::os::raw::c_void;

use rglua::lua::{lua_newuserdata, LuaState, LuaType, Userdata};
use rglua::prelude::{lua_setmetatable, luaL_checkstring, luaL_getmetatable};

use crate::logger::{log, LogLevel};
use crate::mongo::connect_to_db;
use crate::udata::MongoDBClient;

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
        data.write(Box::into_raw(Box::new(MongoDBClient::new(client))) as *mut c_void);
    }

    luaL_getmetatable(l, cstr!("MongoDBClient"));
    lua_setmetatable(l, -2);

    return 1;
}

#[lua_function]
pub fn get_database(_l: LuaState) -> i32 {
    return 1;
}
