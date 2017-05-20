#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate schani_import;

use rocket::Data;
use rocket::request::Form;
use schani_import::*;

#[derive(FromForm)]
struct Import {
    name: String,
}

#[post("/upload", data = "<import>")]
fn upload_data(import: Form<Import>) -> String {
    let imported_file = import.get();
    let conn = establish_db_connection();

    let new_import = create_import(&conn, &imported_file.name);

    format!("import data {} saved for image {}", new_import.id, new_import.name)
}

#[post("/upload/<file_id>", data = "<data>")]
fn upload_image(file_id: &str, data: Data) -> String {
    print!("uploading image {}", file_id);

    // TODO: transfer file to the store
    // TODO: prevent directory traversals
    // TODO: save extension/type on data import
    data.stream_to_file(format!("/tmp/{}.NEF", file_id));

    format!("image {} uploaded successfully", file_id)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![upload_image, upload_data])
        .launch();
}