use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use postgres::{Connection, Error, TlsMode};

const CMD_CREATE: &str = "create";
const CMD_ADD: &str = "add";
const CMD_LIST : &str = "list";

fn main() {
    let cnn_str = "postgres://postgress@localhost:5432";
    let conn = Connection::connect(cnn_str, TlsMode::None).unwrap();
}

fn create_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE USERS (\
                      id SERIAL PRIMARY KEY
                      name VARCHAR NOT NULL,
                      email VARCHAR NOT NULL
   )",
        &[],
    )
    .map(drop)
}

fn create_user(conn: &Connection, name: &str, email: &str) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO users\
         (name, email) \
         VALUES ($1, $2)",
        &[&name, &email],
    )
    .map(drop)
}

fn list_users(conn: &Connection) -> Result<Vec<(String, String)>, Error> {
    let res = conn
        .query("SELECT name, email FROM USERS", &[])?
        .into_iter()
        .map(|row| (row.get(0), row.get(1)))
        .collect();

    Ok(res)
}
