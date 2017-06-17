use super::schema::imports;

#[derive(Serialize)]
#[derive(Queryable)]
pub struct Import {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub camera: String,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Insertable)]
#[table_name="imports"]
pub struct NewImport<'a> {
    pub name: &'a str,
    pub user_id: i32,
    pub camera: &'a str,
    pub longitude: f64,
    pub latitude: f64,
}
