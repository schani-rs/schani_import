use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use futures::Future;
use hyper::Uri;
use tokio_core::reactor::Handle;

// use messaging::send_processing_message;
use models::{Import, NewImport};
use schani_store_client::StoreClient;

#[derive(Clone)]
pub struct ImportService {
    store_uri: Uri,
}

impl ImportService {
    pub fn new(uri: Uri) -> Self {
        ImportService { store_uri: uri }
    }

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

    pub fn create_import<'a>(&self, conn: &PgConnection, new_import: NewImport) -> Import {
        use database::schema::imports;

        // TODO: save extension/type on data import
        let import = diesel::insert_into(imports::table)
            .values(&new_import)
            .get_result(conn)
            .expect("Error saving new import");

        import
    }

    pub fn save_raw_image_id(&self, conn: &PgConnection, import_id: i32, raw_id: String) -> Import {
        use database::schema::imports::dsl::*;
        diesel::update(imports.find(import_id))
            .set(raw_image_id.eq(raw_id))
            .get_result(conn)
            .expect("Could set raw image id")
    }

    pub fn save_sidecar_id(&self, conn: &PgConnection, import_id: i32, sc_id: String) -> Import {
        use database::schema::imports::dsl::*;
        diesel::update(imports.find(import_id))
            .set(sidecar_id.eq(sc_id))
            .get_result(conn)
            .expect("Could not set sidecar id")
    }

    pub fn save_image_id(&self, conn: &PgConnection, import_id: i32, img_id: String) -> Import {
        use database::schema::imports::dsl::*;
        diesel::update(imports.find(import_id))
            .set(image_id.eq(img_id))
            .get_result(conn)
            .expect("Could not set image id")
    }

    pub fn delete_import(&self, conn: &PgConnection, import_id: i32) -> Import {
        use database::schema::imports::dsl::*;

        diesel::delete(imports.filter(id.eq(import_id)))
            .get_result(conn)
            .expect("Could not delete import")
    }

    pub fn add_raw_file(
        &self,
        handle: &Handle,
        data: Vec<u8>,
    ) -> Box<Future<Item = String, Error = ()>> {
        info!("got {} bytes raw image", data.len());

        let store_client = StoreClient::new(self.store_uri.clone(), handle);

        Box::new(store_client.upload_raw_image(data).map_err(|_| ()))
    }

    pub fn add_sidecar(
        &self,
        handle: &Handle,
        data: Vec<u8>,
    ) -> Box<Future<Item = String, Error = ()>> {
        info!("got {} bytes sidecar", data.len());

        let store_client = StoreClient::new(self.store_uri.clone(), handle);

        Box::new(store_client.upload_sidecar(data).map_err(|_| ()))
    }

    pub fn add_image(
        &self,
        handle: &Handle,
        data: Vec<u8>,
    ) -> Box<Future<Item = String, Error = ()>> {
        info!("got {} bytes image", data.len());

        let store_client = StoreClient::new(self.store_uri.clone(), handle);

        Box::new(store_client.upload_image(data).map_err(|_| ()))
    }

    pub fn finish_import(&self, conn: &PgConnection, import_id: i32) -> Import {
        let import = self.delete_import(conn, import_id);

        info!("image {} uploaded successfully", import.id);

        //let image_id = transfer_sidecar_file_to_store(&import, data).expect("transfer failed");

        //info!("image id: {}", image_id);

        //send_processing_message(image_id);

        import
    }
}
