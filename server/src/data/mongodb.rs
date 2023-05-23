use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> (Client, String) {
    dotenv().ok();
    let connection_string: String = env::var("MONGO_DATABASE_URL").expect("MONGO_DATABASE_URL must be set");

    let client_options: ClientOptions = ClientOptions::parse(&connection_string).unwrap();
    let client: Client = Client::with_options(client_options).unwrap();

    let db_name: &str = "chat";
    (client, db_name.to_string())
}