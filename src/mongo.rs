#[cfg(test)]
mod tests {
    use std;
    use std::process::exit;

    use mongodb::Client;
    use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};

    use dotenv::dotenv;

    #[test]
    #[tokio::test]
    async fn test_connect() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        let connection_string = std::env::var("MONGO_CONNECTION_STRING");
        if connection_string.is_err() {
            exit(1);
        }

        let mut client_options = ClientOptions::parse(connection_string).await?;

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;
    }
}