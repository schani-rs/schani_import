extern crate dotenv;
#[macro_use]
extern crate log;
extern crate schani_import;

use std::env;

use dotenv::dotenv;
use schani_import::ImportWebService;

struct ImportData {
    name: String,
    user_id: i32,
    camera: String,
    latitude: f64,
    longitude: f64,
}

/*
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
*/

/*
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
*/

/*
#[post("/upload/<import_id>/sidecar", data = "<data>")]
fn upload_image(import_id: i32, data: Data) -> String {
    info!("uploading image {}", import_id);

    let conn = establish_db_connection();
    let import = finish_import(&conn, import_id, &mut data.open());
    info!("finished import {}", import_id);

    format!("image {} uploaded successfully", import.id)
}
*/

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
    let web_service = ImportWebService::new(database_url.as_str());

    web_service.run();
}
