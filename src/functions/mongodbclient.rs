use mongodb::Client;
use crate::logger::{log, LogLevel};
use crate::mongo::connect_to_db;
use crate::{get_metatable_index, IDENT_MONGODBCLIENT, MONGO_METATABLES};

#[lua_function]
pub unsafe fn new_client(lua: gmod::lua::State) -> i32 {
    lua.get_field(1, lua_string!("connection_url"));
    let connection_url = lua.get_string(1).unwrap();
    log(LogLevel::Debug, &*format!("{}", connection_url));

    let client = connect_to_db(&*connection_url.to_string()).unwrap();

    lua.new_userdata(client, Option::from(get_metatable_index(IDENT_MONGODBCLIENT.clone())));

    return 1;
}

#[lua_function]
pub unsafe fn get_database(lua: gmod::lua::State) -> i32 {
    lua.get_field(1, lua_string!("database_name"));
    let database_name = lua.get_string(1).unwrap();
    log(
        LogLevel::Debug,
        &*format!("Database Name: {}", database_name),
    );
    return 1;
}
