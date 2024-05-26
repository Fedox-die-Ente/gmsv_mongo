use std::ffi::c_void;
use std::sync::{Arc, mpsc, Mutex};

use mongodb::Client;
use mongodb::options::ClientOptions;
use rglua::lua::LuaState;
use rglua::prelude::{lua_call, lua_newuserdata, lua_setmetatable, luaL_checkstring, luaL_checkudata, luaL_getmetatable};

use crate::logger::{log, LogLevel};

async fn connect_to_db(connection_url: String, sender: mpsc::Sender<Result<Arc<Mutex<Client>>, mongodb::error::Error>>) {
    let client_options = ClientOptions::parse(&connection_url).await;
    match client_options {
        Ok(options) => {
            let client = Client::with_options(options);
            match client {
                Ok(client) => {
                    let client = Arc::new(Mutex::new(client));
                    let _ = sender.send(Ok(client));
                }
                Err(e) => {
                    let _ = sender.send(Err(e));
                }
            }
        }
        Err(e) => {
            let _ = sender.send(Err(e));
        }
    }
}

async fn connect(connection_url: String, l: LuaState) {
    let client = Client::with_options(ClientOptions::parse(&connection_url).await.unwrap()).unwrap();
    let client = Arc::new(Mutex::new(client));

    unsafe {
        let ptr = lua_newuserdata(l, std::mem::size_of::<Arc<Mutex<Client>>>());
        let data = std::ptr::addr_of_mut!((*ptr).data);
        data.write(Box::into_raw(Box::new(client)) as *mut c_void);
    }

    luaL_getmetatable(l, cstr!("MongoDBClient"));
    lua_setmetatable(l, -2);
    lua_call(l, 1, 0);
}

#[lua_function]
pub fn new_client(l: LuaState) -> i32 {
    let connection_url = rstr!(luaL_checkstring(l, 1));
    log(LogLevel::Info, &format!("Connecting to MongoDB at {:?}", connection_url));

    let client_options = tokio::task::block_in_place(|| {
        let (sender, receiver) = mpsc::channel();
        tokio::spawn(connect_to_db((&*connection_url).to_string(), sender));
        receiver.recv().unwrap()
    });

    return 0;
}

#[lua_function]
pub fn get_database(l: LuaState) -> i32 {
    let client = unsafe {
        let ptr = luaL_checkudata(l, 1, cstr!("MongoDBClient")) as *mut Client;
        Box::from_raw(ptr)
    }.clone();

    let database_name = rstr!(luaL_checkstring(l, 2));
    log(LogLevel::Info, &format!("Retrieving database '{}'...", database_name));

    let db = client.database(&*database_name);
    log(LogLevel::Info, &format!("Database '{}' retrieved.", database_name));
    return 1;
}

