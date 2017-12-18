use database::schema::imports;

#[derive(Serialize, Queryable)]
pub struct Import {
    pub id: i32,
    pub title: Option<String>,
    pub raw_image_id: Option<String>,
    pub sidecar_id: Option<String>,
    pub image_id: Option<String>,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "imports"]
pub struct NewImport {
    pub title: Option<String>,
    pub user_id: i32,
}
