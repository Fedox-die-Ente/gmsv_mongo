#[lua_function]
pub unsafe fn insert(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.get_field(1, lua_string!("_connection_url"));
    let database_name = lua.get_field(1, lua_string!("_database_name"));
    let collection_name = lua.get_field(1, lua_string!("_collection_name"));
    let document = lua.check_string(1);

    return 1;
}

#[lua_function]
pub unsafe fn find(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.get_field(1, lua_string!("_connection_url"));
    let database_name = lua.get_field(1, lua_string!("_database_name"));
    let collection_name = lua.get_field(1, lua_string!("_collection_name"));
    let query = lua.check_string(1);

    return 1;
}

#[lua_function]
pub unsafe fn update(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.get_field(1, lua_string!("_connection_url"));
    let database_name = lua.get_field(1, lua_string!("_database_name"));
    let collection_name = lua.get_field(1, lua_string!("_collection_name"));
    let query = lua.check_string(1);
    let update = lua.check_string(1);

    return 1;
}

#[lua_function]
pub unsafe fn delete(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.get_field(1, lua_string!("connection_url"));
    let database_name = lua.get_field(1, lua_string!("database_name"));
    let collection_name = lua.get_field(1, lua_string!("collection_name"));
    let query = lua.check_string(1);

    return 1;
}