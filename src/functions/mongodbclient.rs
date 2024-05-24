use crate::functions::mongodbdatabase::get_collection;

#[lua_function]
pub unsafe fn new_client(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.check_string(1);

    lua.push_string(&*connection_url);
    lua.set_field(-2, lua_string!("connection_url"));

    lua.push_function(get_database);
    lua.set_field(-2, lua_string!("GetDatabase"));

    return 1;
}

#[lua_function]
pub unsafe fn get_database(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.get_field(1, lua_string!("connection_url"));
    let database_name = lua.check_string(1);

    lua.push_string(&*database_name);
    lua.set_field(-2, lua_string!("database_name"));
    lua.push_function(get_collection);
    lua.set_field(-2, lua_string!("GetCollection"));

    return 1;
}
