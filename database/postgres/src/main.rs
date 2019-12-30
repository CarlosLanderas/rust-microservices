use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use postgres::{Connection, Error, TlsMode};
use core::num::FpCategory::Subnormal;

const CMD_CREATE: &str = "create";
const CMD_ADD: &str = "add";
const CMD_LIST : &str = "list";

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("db")
                .value_name("ADDR")
                .help("Sets and address of database connection")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name(CMD_CREATE).about("crate users table"))
        .subcommand(SubCommand::with_name(CMD_ADD)).about("add user to the table")
        .arg(Arg::with_name("NAME")
            .help("Sets the name of a user")
            .required(true)
            .index(1))
        .arg(Arg::with_name("EMAIL")
            .help("Sets the email of a user")
            .required(true)
            .index(2))
        .subcommand(SubCommand::with_name(CMD_LIST)).about("print list of users")
        .get_matches();

    let addr = matches.value_of("database")
        .unwrap_or("postgres://postgress@locahost:5432");

    let conn = Connection::connect(addr, TlsMode::None).unwrap();

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
