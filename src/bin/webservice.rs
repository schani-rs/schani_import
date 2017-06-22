#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate log;
extern crate rocket;
extern crate rocket_contrib;
extern crate schani_import;

use rocket::Data;
use rocket_contrib::JSON;
use rocket::request::Form;
use schani_import::*;
use schani_import::models::Import;

#[derive(FromForm)]
struct ImportData {
    name: String,
    user_id: i32,
    camera: String,
    latitude: f64,
    longitude: f64,
}

#[post("/upload", data = "<import>")]
fn upload_data(import: Form<ImportData>) -> JSON<Import> {
    let imported_file: &ImportData = import.get();
    let conn = establish_db_connection();

    let new_import = create_import(
        &conn,
        &imported_file.name,
        imported_file.user_id,
        &imported_file.camera,
        imported_file.latitude,
        imported_file.longitude,
    );

    info!("import {} initiazed", new_import.id);

    JSON(new_import)
}

#[post("/upload/<import_id>/raw", data = "<data>")]
fn upload_raw_image(import_id: i32, data: Data) -> String {
    info!("uploading raw image {}", import_id);

    let conn = establish_db_connection();
    let import = add_raw_file(&conn, import_id, &mut data.open());
    info!(
        "raw image {} saved for import {}",
        import.raw_image_id.unwrap(),
        import_id
    );

    format!("raw image {} uploaded successfully", import.id)
}

#[post("/upload/<import_id>/sidecar", data = "<data>")]
fn upload_image(import_id: i32, data: Data) -> String {
    info!("uploading image {}", import_id);

    let conn = establish_db_connection();
    let import = finish_import(&conn, import_id, &mut data.open());
    info!("finished import {}", import_id);

    format!("image {} uploaded successfully", import.id)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![upload_data, upload_raw_image, upload_image])
        .launch();
}
