#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

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
}

#[post("/upload", data = "<import>")]
fn upload_data(import: Form<ImportData>) -> JSON<Import> {
    let imported_file = import.get();
    let conn = establish_db_connection();

    let new_import = create_import(&conn, &imported_file.name);

    JSON(new_import)
}

#[post("/upload/<import_id>", data = "<data>")]
fn upload_image(import_id: i32, data: Data) -> String {
    println!("uploading image {}", import_id);

    let conn = establish_db_connection();
    let import = finish_import(&conn, import_id, &mut data.open());

    format!("image {} uploaded successfully", import.id)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![upload_image, upload_data])
        .launch();
}