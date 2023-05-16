mod server;
mod schema;

mod data {
    pub mod postgresql;
}
mod models {
    pub mod users;
}
mod resolvers {
    pub mod users_resolver;
}

fn main() {
    println!("Hello, world!");
}
