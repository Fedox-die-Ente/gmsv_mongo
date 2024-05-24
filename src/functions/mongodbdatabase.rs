#![allow(dead_code)]

use crate::functions::mongodbcollection::{delete, find, insert, update};

#[lua_function]
pub unsafe fn get_collection(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.get_field(1, lua_string!("_connection_url"));
    let database_name = lua.get_field(1, lua_string!("_database_name"));
    let collection_name = lua.get_string(1).unwrap();

    lua.push_string(&*collection_name);
    lua.set_field(-2, lua_string!("collection_name"));
    lua.push_function(insert);
    lua.set_field(-2, lua_string!("Insert"));
    lua.push_function(find);
    lua.set_field(-2, lua_string!("Find"));
    lua.push_function(update);
    lua.set_field(-2, lua_string!("Update"));
    lua.push_function(delete);
    lua.set_field(-2, lua_string!("Delete"));

    return 1;
}