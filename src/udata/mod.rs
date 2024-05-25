pub struct MongoDBClient {
    pub client: mongodb::Client,
}

pub struct MongoDBDatabase {
    pub database: mongodb::Database,
}

pub struct MongoDBCollection {
    pub collection: mongodb::Collection<mongodb::bson::Document>,
}