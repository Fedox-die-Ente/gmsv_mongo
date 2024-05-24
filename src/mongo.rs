use mongodb::{Client, Database};
use mongodb::bson::Document;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};

use crate::logger::{log, LogLevel};

// ██████╗  █████╗ ████████╗ █████╗ ██████╗  █████╗ ███████╗███████╗
// ██╔══██╗██╔══██╗╚══██╔══╝██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝
// ██║  ██║███████║   ██║   ███████║██████╔╝███████║███████╗█████╗
// ██║  ██║██╔══██║   ██║   ██╔══██║██╔══██╗██╔══██║╚════██║██╔══╝
// ██████╔╝██║  ██║   ██║   ██║  ██║██████╔╝██║  ██║███████║███████╗
// ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝

pub(crate) async fn connect_to_db(connection_string: &str) -> mongodb::error::Result<Client> {
    if connection_string.is_empty() {
        log(LogLevel::Error, "MONGO_CONNECTION_STRING must be set");
        panic!("MONGO_CONNECTION_STRING must be set");
    }

    let mut client_options = ClientOptions::parse(connection_string).await.expect("Failed to parse connection string.");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options);

    if client.is_ok() {
        log(LogLevel::Info, "Connected to MongoDB");
        return client;
    } else {
        log(LogLevel::Error, "Failed to connect to MongoDB");
    }

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

pub(crate) async fn check_if_collection_exists(db: &Database, collection_name: &str) -> Result<String, String> {
    let collection_names = db.list_collection_names(None).await;

    match collection_names {
        Ok(names) => {
            if names.contains(&collection_name.to_string()) {
                Ok(collection_name.to_string())
            } else {
                Err("Collection doesn't exist.".to_string())
            }
        }
        Err(_) => Err("Failed to retrieve collection names.".to_string()),
    }
}

pub(crate) async fn create_collection(db: &Database, collection_name: &str) {
    if check_if_collection_exists(db, collection_name).await.is_ok() {
        let txt = format!("Collection '{}' already exists in database '{}'.", collection_name, db.name());
        log(LogLevel::Error, &*txt);
        return;
    }

    let txt = format!("Collection '{}' created in database '{}'.", collection_name, db.name());
    log(LogLevel::Info, txt.as_str());
    db.create_collection(collection_name, None).await.expect("Failed to create collection");
}

pub(crate) async fn drop_collection(database: &Database, collection_name: &str) -> mongodb::error::Result<()> {
    if check_if_collection_exists(database, collection_name).await.is_err() {
        let txt = format!("Failed to drop collection '{}' from database '{}'. Reason: The collection doesn't exist.", collection_name, database.name());
        log(LogLevel::Error, txt.as_str());
        return Ok(());
    }

    database.collection::<Document>(collection_name).drop(None).await.expect("Failed to drop collection");
    let txt = format!("Collection '{}' dropped from database '{}'.", collection_name, database.name());
    log(LogLevel::Info, txt.as_str());

    return Ok(());
}

pub(crate) async fn get_collection(database: &Database, collection_name: &str) -> mongodb::error::Result<Option<mongodb::Collection<Document>>> {
    let collection_names = database.list_collection_names(None).await?;

    if collection_names.contains(&collection_name.to_string()) {
        Ok(Some(database.collection(collection_name)))
    } else {
        Ok(None)
    }
}