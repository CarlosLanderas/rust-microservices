#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::{insert_into, Connection, ExpressionMethods, PgConnection, RunQueryDsl};
use failure::{format_err, Error};
use models::{Channel, Id, Membership, Message, User};
use schema::{channels, memberships, messages, users};
use std::env;

pub struct Api {
    conn: PgConnection,
}

impl Api {
    pub fn connect() -> Result<Self, Error> {
        let database_url =
            env::var("DATABASE_URL").unwrap_or("postgres//postgres@localhost:5432".to_string());
        let conn = PgConnection::establish(&database_url)?;
        Ok(Self { conn })
    }

    pub fn register_user(&self, email: &str) -> Result<User, Error> {
        insert_into(users::table)
            .values((users::email.eq("email")))
            .returning((users::id, users::email))
            .get_result(&self.conn)
            .map_err(Error::from)
    }
}
