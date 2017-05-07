extern crate schani_import;
extern crate diesel;

use std::env::args;

use self::schani_import::{establish_db_connection, create_import};

fn main() {
    let connection = establish_db_connection();

    let name = args().nth(1).expect("create_import requires a name")
        .parse::<String>().expect("Invalid name");
    let import = create_import(&connection, &name);

    println!("Created import {}: {}", import.id, import.name);
}