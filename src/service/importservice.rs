use std::io::Read;

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

// use messaging::send_processing_message;
use models::{Import, NewImport};
// use service::store::transfer_raw_image_to_store;

pub struct ImportService {}

impl ImportService {
    pub fn get_imports(&self, conn: &PgConnection) -> Vec<Import> {
        use database::schema::imports::dsl::*;

        imports.load::<Import>(conn).expect("Error loading imports")
    }

    pub fn get_import(&self, conn: &PgConnection, import_id: &i32) -> Import {
        use database::schema::imports::dsl::*;

        imports
            .find(import_id)
            .get_result(conn)
            .expect(&format!("Error loading import with id={}", import_id))
    }

    pub fn create_import<'a>(
        &self,
        conn: &PgConnection,
        name: &'a str,
        user_id: i32,
        camera: &'a str,
        latitude: f64,
        longitude: f64,
    ) -> Import {
        use database::schema::imports;

        let new_import = NewImport {
            name: name,
            user_id: user_id,
            camera: camera,
            latitude: latitude,
            longitude: longitude,
        };

        // TODO: save extension/type on data import
        let import = diesel::insert_into(imports::table)
            .values(&new_import)
            .get_result(conn)
            .expect("Error saving new import");

        import
    }

    pub fn save_raw_image_id(&self, conn: &PgConnection, import_id: i32, raw_id: i32) -> Import {
        use database::schema::imports::dsl::*;
        diesel::update(imports.find(import_id))
            .set(raw_image_id.eq(raw_id))
            .get_result(conn)
            .expect("Could not delete import")
    }

    pub fn delete_import(&self, conn: &PgConnection, import_id: i32) -> Import {
        use database::schema::imports::dsl::*;

        diesel::delete(imports.filter(id.eq(import_id)))
            .get_result(conn)
            .expect("Could not delete import")
    }

    pub fn add_raw_file(&self, conn: &PgConnection, import_id: i32, data: &mut Read) -> Import {
        let import = self.get_import(conn, &import_id);

        //let raw_image_id = transfer_raw_image_to_store(&import, data).expect("transfer failed");
        //info!("transferred raw image: {}", raw_image_id);
        //let import = save_raw_image_id(conn, import_id, raw_image_id);

        import
    }

    pub fn finish_import(&self, conn: &PgConnection, import_id: i32, data: &mut Read) -> Import {
        let import = self.delete_import(conn, import_id);

        info!("image {} uploaded successfully", import.name);

        //let image_id = transfer_sidecar_file_to_store(&import, data).expect("transfer failed");

        //info!("image id: {}", image_id);

        //send_processing_message(image_id);

        import
    }
}
