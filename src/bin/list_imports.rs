extern crate schani_import;
extern crate diesel;

use self::schani_import::*;
use self::schani_import::models::*;
use self::diesel::prelude::*;

fn main() {
    use schani_import::schema::imports::dsl::*;

    let connection = establish_connection();
    let results = imports.filter(id.eq(1))
        .load::<Import>(&connection)
        .expect("Error loading imports");

    println!("Found {} imports", results.len());
    for import in results {
        println!("{}", import.name);
        println!("----------\n");
    }
}