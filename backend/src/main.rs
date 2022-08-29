use diesel::*;

use crate::db_connect::establish_connection;

#[macro_use]
extern crate diesel;
use nanoid::nanoid;

extern crate dotenv;

pub mod db_connect;
pub mod insertable_models;
pub mod models;
pub mod schema;

fn main() {
    use self::schema::users::dsl::*;
    let connection = establish_connection();

    let ID = nanoid!().as_str();

    let u: insertable_models::NewUser = insertable_models::NewUser {
        name: "ashtyn",
        uuid: ID,
        email: "email",
        enc_key: "key",
        customer_id: "id",
    };

    diesel::insert_into(schema::users::table)
        .values(&u)
        .execute(&connection)
        .expect("bad insert");

    let all_users = users
        .load::<models::User>(&connection)
        .expect("Error loading users");

    println!("User count: {:?}", all_users.len());
    // println!("{:?}", res[0]);
}
