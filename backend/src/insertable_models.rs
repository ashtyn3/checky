use super::schema::users;
use diesel_codegen::Insertable;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub enc_key: &'a str,
    pub customer_id: &'a str,
}
