use clap::{
    crate_authors, crate_description, crate_name, crate_version,
    App, AppSettings, Arg,SubCommand
};
use postgres::{Connection, TlsMode, Error};

fn main() {
    let cnn_str = "postgres://postgress@localhost:5432";
    let conn = match Connection::connect(cnn_str, TlsMode::None).unwrap();


}

fn create_table(conn: &Connection) -> Result<(), Error> {
   conn.execute("CREATE TABLE USERS (\
                      id SERIAL PRIMARY KEY
                      name VARCHAR NOT NULL,
                      email VARCHAR NOT NULL
   )", &[])
       .map(drop)
}
