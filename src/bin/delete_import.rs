extern crate schani_import;
extern crate diesel;

use std::env::args;

use self::schani_import::{establish_db_connection, delete_import};

fn main() {
    let connection = establish_db_connection();

    let id = args().nth(1).expect("delete_import requires an import id")
        .parse::<i32>().expect("Invalid ID");
    let import = delete_import(&connection, id);

    println!("Deleted import {}", import.name);
}