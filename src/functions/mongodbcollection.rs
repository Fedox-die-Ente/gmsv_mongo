#[lua_function]
pub unsafe fn insert(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.get_field::<String>(1, lua_string!("connection_url")).unwrap();
    let database_name = lua.get_field::<String>(1, lua_string!("database_name")).unwrap();
    let collection_name = lua.get_field::<String>(1, lua_string!("collection_name")).unwrap();
    let document = lua.check_string(1);

    return 1;
}

#[lua_function]
pub unsafe fn find(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.get_field::<String>(1, lua_string!("connection_url")).unwrap();
    let database_name = lua.get_field::<String>(1, lua_string!("database_name")).unwrap();
    let collection_name = lua.get_field::<String>(1, lua_string!("collection_name")).unwrap();
    let query = lua.check_string(1);

    return 1;
}

#[lua_function]
pub unsafe fn update(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.get_field::<String>(1, lua_string!("connection_url")).unwrap();
    let database_name = lua.get_field::<String>(1, lua_string!("database_name")).unwrap();
    let collection_name = lua.get_field::<String>(1, lua_string!("collection_name")).unwrap();
    let query = lua.check_string(1);
    let update = lua.check_string(1);

    return 1;
}

#[lua_function]
pub unsafe fn delete(lua: gmod::lua::State) -> i32 {
    let connection_url = lua.get_field::<String>(1, lua_string!("connection_url")).unwrap();
    let database_name = lua.get_field::<String>(1, lua_string!("database_name")).unwrap();
    let collection_name = lua.get_field::<String>(1, lua_string!("collection_name")).unwrap();
    let query = lua.check_string(1);

    return 1;
}