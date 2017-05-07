#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

use self::models::Import;

pub fn establish_db_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_imports(conn: &MysqlConnection) -> Vec<Import> {
    use self::schema::imports::dsl::*;

    imports.load::<Import>(conn)
        .expect("Error loading imports")
}
