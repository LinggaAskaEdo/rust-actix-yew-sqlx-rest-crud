use sqlx::MySqlPool;

mod handler;
mod model;
mod request;
mod response;

pub struct AppState {
    db: MySqlPool,
}

fn main() {
    println!("Hello, world!");
}
