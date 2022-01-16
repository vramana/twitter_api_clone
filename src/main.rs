extern crate bcrypt;
extern crate chrono;
extern crate dotenv;
extern crate eyre;
extern crate r2d2;
extern crate tide;

#[macro_use]
extern crate diesel;

use dotenv::dotenv;

use tide::prelude::*;

use std::env;
use std::sync::Arc;

mod db;
mod models;
mod schema;

use crate::db::*;
use crate::models::*;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[derive(Clone)]
pub struct Application {
    pub db: Arc<PgDB>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let app_state = Application {
        db: Arc::new(Database::new(&env::var("DATABASE_URL")?)),
    };
    let mut server = tide::with_state(app_state);
    server.at("/users").post(users::create_user);
    server.listen("127.0.0.1:8000").await?;
    Ok(())
}
