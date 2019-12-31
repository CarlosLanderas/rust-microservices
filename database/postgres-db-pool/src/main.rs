use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use csv::{ReaderBuilder, StringRecord};
use postgres::{Connection, Error};
use r2d2_postgres::PostgresConnectionManager;
use rayon::prelude::*;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}

impl User {
    pub fn new(name: String, email: String) -> User {
        User { name, email }
    }
}

const CMD_CREATE: &str = "create";
const CMD_ADD: &str = "add";
const CMD_LIST: &str = "list";
const CMD_IMPORT: &str = "import";

fn main() -> Result<(), failure::Error> {
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
        .subcommand(SubCommand::with_name(CMD_CREATE).about("create users table"))
        .subcommand(
            SubCommand::with_name(CMD_ADD)
                .about("add user to the table")
                .arg(
                    Arg::with_name("NAME")
                        .help("Sets the name of a user")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("EMAIL")
                        .help("Sets the email of a user")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(SubCommand::with_name(CMD_LIST).about("print list of users"))
        .subcommand(SubCommand::with_name(CMD_IMPORT).about("import users from csv"))
        .get_matches();

    let addr = matches
        .value_of("database")
        .unwrap_or("postgres://admin:example@localhost:5432");

    let manager = PostgresConnectionManager::new(addr, r2d2_postgres::TlsMode::None)?;
    let pool = r2d2::Pool::new(manager)?;
    let conn = pool.get()?;

    match matches.subcommand() {
        (CMD_CREATE, _) => {
            create_table(&conn)?;
        }
        (CMD_ADD, Some(matches)) => {
            let name = matches.value_of("NAME").unwrap().to_owned();
            let email = matches.value_of("EMAIL").unwrap().to_owned();
            create_user(&conn, &User::new(name, email))?;
        }
        (CMD_LIST, _) => {
            let list = list_users(&conn)?;
            for user in list {
                println!("Name: {:20} - Email {:20}", user.name, user.email);
            }
        }
        (CMD_IMPORT, _) => {
            let mut reader = ReaderBuilder::new()
                .has_headers(true)
                .from_path("users.csv")?;

            let header = StringRecord::from(vec!["name", "email"]);
            let mut users = Vec::new();

            for row in reader.records() {
                let user: User = row?.deserialize(Some(&header))?;
                users.push(User::new(user.name, user.email));
            }

            users
                .par_iter()
                .map(|user| -> Result<(), failure::Error> {
                    let conn = pool.get()?;
                    create_user(&conn, &user)?;
                    Ok(())
                })
                .for_each(drop);
        }
        _ => {
            matches.usage();
        }
    }

    Ok(())
}

fn create_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE users (
                      id SERIAL PRIMARY KEY,
                      name VARCHAR NOT NULL,
                      email VARCHAR NOT NULL
        )",
        &[],
    )
    .map(drop)
}

fn create_user(conn: &Connection, user: &User) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO users\
         (name, email) \
         VALUES ($1, $2)",
        &[&user.name, &user.email],
    )
    .map(drop)
}

fn list_users(conn: &Connection) -> Result<Vec<User>, Error> {
    let res = conn
        .query("SELECT name, email FROM USERS", &[])?
        .into_iter()
        .map(|row| User::new(row.get(0), row.get(1)))
        .collect();

    Ok(res)
}
