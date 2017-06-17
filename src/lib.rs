#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde_urlencoded;
extern crate tokio_core;

pub mod models;
pub mod schema;
mod messaging;
mod store;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io::Read;

use messaging::send_processing_message;
use models::{Import, NewImport};
use store::transfer_image_to_store;

pub fn establish_db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_imports(conn: &PgConnection) -> Vec<Import> {
    use self::schema::imports::dsl::*;

    imports.load::<Import>(conn).expect("Error loading imports")
}

pub fn get_import(conn: &PgConnection, import_id: &i32) -> Import {
    use self::schema::imports::dsl::*;

    imports
        .find(import_id)
        .get_result(conn)
        .expect(&format!("Error loading import with id={}", import_id))
}

pub fn create_import<'a>(conn: &PgConnection,
                         name: &'a str,
                         user_id: i32,
                         camera: &'a str,
                         latitude: f64,
                         longitude: f64)
                         -> Import {
    use schema::imports;

    let new_import = NewImport {
        name: name,
        user_id: user_id,
        camera: camera,
        latitude: latitude,
        longitude: longitude,
    };

    // TODO: save extension/type on data import
    let import = diesel::insert(&new_import)
        .into(imports::table)
        .get_result(conn)
        .expect("Error saving new import");

    import
}

pub fn delete_import(conn: &PgConnection, import_id: i32) -> Import {
    use self::schema::imports::dsl::*;

    diesel::delete(imports.filter(id.eq(import_id)))
        .get_result(conn)
        .expect("Could not delete import")
}

pub fn finish_import(conn: &PgConnection, import_id: i32, data: &mut Read) -> Import {
    let import = delete_import(conn, import_id);

    println!("Image {} uploaded successfully", import.name);

    let image_id = transfer_image_to_store(&import, data).expect("transfer failed");

    println!("image hash: {}", image_id);

    send_processing_message(image_id);

    import
}
