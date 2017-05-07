extern crate schani_import;
extern crate diesel;

use self::schani_import::{establish_db_connection, get_imports};

fn main() {
    let connection = establish_db_connection();

    let results = get_imports(&connection);

    println!("Found {} imports:", results.len());
    for import in results {
        println!("  {}: {}", import.id, import.name);
    }
}
