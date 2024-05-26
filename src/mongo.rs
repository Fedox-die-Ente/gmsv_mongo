#![allow(dead_code)]

use lazy_static::lazy_static;
use mongodb::{Client};
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use tokio::runtime::Runtime;


// ██████╗  █████╗ ████████╗ █████╗ ██████╗  █████╗ ███████╗███████╗
// ██╔══██╗██╔══██╗╚══██╔══╝██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝
// ██║  ██║███████║   ██║   ███████║██████╔╝███████║███████╗█████╗
// ██║  ██║██╔══██║   ██║   ██╔══██║██╔══██╗██╔══██║╚════██║██╔══╝
// ██████╔╝██║  ██║   ██║   ██║  ██║██████╔╝██║  ██║███████║███████╗
// ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝

lazy_static! {
    pub static ref MONGO_WORKER: Runtime = Runtime::new().unwrap();
}

pub fn create_client_options(connection_url: String) -> ClientOptions {
    MONGO_WORKER.block_on(async {
        let mut client_options = ClientOptions::parse(connection_url).await.unwrap();
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        client_options.server_selection_timeout = Some(std::time::Duration::from_secs(30));
        client_options
    })
}

pub fn create_mongo_client(client_options: ClientOptions) -> Client {
    let client = MONGO_WORKER.block_on(async {
        Client::with_options(client_options).expect("Failed to create client")
    });
    return client;
}

pub(crate) fn get_database(client: Client, database_name: &str) -> mongodb::Database {
    let db = client.database(database_name);

    return db;
}

//   ██████╗ ██████╗ ██╗     ██╗     ███████╗ ██████╗████████╗██╗ ██████╗ ███╗   ██╗███████╗
//  ██╔════╝██╔═══██╗██║     ██║     ██╔════╝██╔════╝╚══██╔══╝██║██╔═══██╗████╗  ██║██╔════╝
//  ██║     ██║   ██║██║     ██║     █████╗  ██║        ██║   ██║██║   ██║██╔██╗ ██║███████╗
//  ██║     ██║   ██║██║     ██║     ██╔══╝  ██║        ██║   ██║██║   ██║██║╚██╗██║╚════██║
//  ╚██████╗╚██████╔╝███████╗███████╗███████╗╚██████╗   ██║   ██║╚██████╔╝██║ ╚████║███████║
//   ╚═════╝ ╚═════╝ ╚══════╝╚══════╝╚══════╝ ╚═════╝   ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝

// pub(crate) fn check_if_collection_exists(db: &Database, collection_name: &str) -> Result<String, String> {
//     let collection_names = db.list_collection_names(None);
//
//     match collection_names {
//         Ok(names) => {
//             return if names.contains(&collection_name.to_string()) {
//                 true
//             } else {
//                 false
//             }
//         }
//         Err(_) => Err("Failed to retrieve collection names.".to_string()),
//     }
// }

// pub(crate) async fn create_collection(db: &Database, collection_name: &str) {
//     if check_if_collection_exists(db, collection_name).await.is_ok() {
//         let txt = format!("Collection '{}' already exists in database '{}'.", collection_name, db.name());
//         log(LogLevel::Error, &*txt);
//         return;
//     }
//
//     let txt = format!("Collection '{}' created in database '{}'.", collection_name, db.name());
//     log(LogLevel::Info, txt.as_str());
//     db.create_collection(collection_name, None).await.expect("Failed to create collection");
// }

// pub(crate) async fn drop_collection(database: &Database, collection_name: &str) -> mongodb::error::Result<()> {
//     if check_if_collection_exists(database, collection_name).await.is_err() {
//         let txt = format!("Failed to drop collection '{}' from database '{}'. Reason: The collection doesn't exist.", collection_name, database.name());
//         log(LogLevel::Error, txt.as_str());
//         return Ok(());
//     }
//
//     database.collection::<Document>(collection_name).drop(None).await.expect("Failed to drop collection");
//     let txt = format!("Collection '{}' dropped from database '{}'.", collection_name, database.name());
//     log(LogLevel::Info, txt.as_str());
//
//     return Ok(());
// }

// pub(crate) async fn get_collection(database: &Database, collection_name: &str) -> mongodb::error::Result<Option<mongodb::Collection<Document>>> {
//     let collection_names = database.list_collection_names(None).await?;
//
//     if collection_names.contains(&collection_name.to_string()) {
//         Ok(Some(database.collection(collection_name)))
//     } else {
//         Ok(None)
//     }
// }