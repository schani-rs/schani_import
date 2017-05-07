use super::schema::imports;

#[derive(Queryable)]
pub struct Import {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="imports"]
pub struct NewImport<'a> {
    pub name: &'a str,
}
