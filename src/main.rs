extern crate dotenv;
extern crate log;
extern crate schani_import;

use std::env;

use dotenv::dotenv;
use schani_import::ImportWebService;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
    let library_url = env::var("LIBRARY_URL").expect("LIBRARY_URL required");
    let store_url = env::var("STORE_URL").expect("STORE_URL required");
    let web_service = ImportWebService::new(&database_url, &library_url, &store_url);

    web_service.run();
}
