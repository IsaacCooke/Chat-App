mod server;
mod schema;

mod data {
    pub mod postgresql;
    pub mod mongodb;
}
mod models {
    pub mod users;
    pub mod message;
    pub mod chat;
}
mod resolvers {
    pub mod graphql_schema;
    pub mod users_resolver;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run_server().await
}