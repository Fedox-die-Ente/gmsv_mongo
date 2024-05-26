#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use mongodb::{Client, Collection};
    use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
    use serde::{Deserialize, Serialize};

    use crate::logger::{log, LogLevel};
    use crate::mongo::{craete_client_options, create_mongo_client};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestType {
        name: String,
        age: i32,
    }

    #[tokio::test]
    async fn test_insert() -> mongodb::error::Result<()> {
        dotenv().ok();

        let connection_string = dotenv::var("MONGO_CONNECTION_STRING").expect("MONGODB_URI must be set");
        if connection_string.is_empty() {
            panic!("MONGO_CONNECTION_STRING must be set");
        }

        let mut client_options = ClientOptions::parse(connection_string).await.unwrap();
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;

        let db = client.database("admin");

        let _ = db.create_collection("testtest", None).await?;
        log(LogLevel::Info, "Collection created.");

        let collection: Collection<TestType> = db.collection("testtest");

        let test = TestType {
            name: "Test".to_string(),
            age: 20,
        };

        collection.insert_one(test, None).await.expect("Failed to insert");
        collection.drop(None).await.expect("Failed to drop collection");

        Ok(())
    }

    #[test]
    fn test_connect() -> mongodb::error::Result<()> {
        dotenv().ok();

        let connection_string = dotenv::var("MONGO_CONNECTION_STRING").expect("MONGODB_URI must be set");
        if connection_string.is_empty() {
            panic!("MONGO_CONNECTION_STRING must be set");
        }

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let receiver = craete_client_options(connection_string);

        match receiver.recv().unwrap() {
            Ok(client_options) => {
                let client = create_mongo_client(client_options);
                let admin_db = client.database("admin");
                assert_eq!(admin_db.name(), "admin");
            }
            Err(e) => {
                panic!("Failed to connect to MongoDB: {}", e);
            }
        }
        Ok(())
    }
}
