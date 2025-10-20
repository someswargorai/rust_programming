use mongodb::{Client, Database, options::ClientOptions};

pub async fn get_db() -> Database {
    
    let client_uri: &'static str = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(client_uri).await.unwrap_or_default();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database("mydb");
    db
}