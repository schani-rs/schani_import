extern crate dotenv;
extern crate log;
extern crate schani_import;

use std::env;

use dotenv::dotenv;
use schani_import::ImportWebService;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
    let web_service = ImportWebService::new(database_url.as_str());

    web_service.run();
}
