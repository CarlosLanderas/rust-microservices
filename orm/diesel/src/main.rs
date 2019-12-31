#[macro_use]
extern crate diesel;
pub mod schema;
pub mod models;
use std::env;
use failure::Error;
use diesel::r2d2::ConnectionManager;
use diesel::{SqliteConnection, RunQueryDsl, QueryDsl, TextExpressionMethods};
use self::schema::users::dsl::*;

fn main() -> Result<(), Error> {

    let path = env::var("database").unwrap_or("test.db".to_owned());
    let manager = ConnectionManager::<SqliteConnection>::new(path);
    let pool = r2d2::Pool::new(manager)?;
    let conn = pool.get()?;

    let user1 = models::NewUser {
        id: &format!("{}", uuid::Uuid::new_v4()),
        name: "Carlos",
        email: "carlos@host.com"
    };

    let user2 = models::NewUser {
        id: &format!("{}", uuid::Uuid::new_v4()),
        name: "Geralt",
        email: "geralt@derivia.com"
    };

    //Insert users
    diesel::insert_into(schema::users::table)
        .values(&user1)
        .execute(&conn)?;

    diesel::insert_into(schema::users::table)
        .values(&user2)
        .execute(&conn)?;

    //List users
    println!("Listing users...");
    let items = users.load::<models::User>(&conn)?;
    for user in items {
        println!("{:?}", user);
    }

    println!("Listing with filter");
    let items = users.filter(email.like("%@derivia.com%"))
        .limit(10)
        .load::<models::User>(&conn)?;

    for user in items {
        println!("{:?}", user);
    }

    Ok(())

}
