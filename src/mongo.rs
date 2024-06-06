use lazy_static::lazy_static;
use mongodb::Client;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use tokio::runtime::Runtime;

lazy_static! {
    pub static ref MONGO_WORKER: Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
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
