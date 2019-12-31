use bson::{bson, doc};
use mongodb::{
    Error,
    db::{Database, ThreadedDatabase}
};

use serde_derive::Deserialize;
use clap::{SubCommand, App, AppSettings, Arg, crate_name, crate_version, crate_authors, crate_description};
use url::Url;
use r2d2_mongodb::{ConnectionOptionsBuilder, MongodbConnectionManager};
use r2d2::Pool;
use chrono::Utc;

const CMD_ADD: &str = "add";
const CMD_LIST: &str = "list";

#[derive(Debug, Deserialize)]
struct Activity {
    user_id: String,
    activity: String,
    datetime: String,
}

fn add_activity(conn: &Database, activity: Activity) -> Result<(),Error> {
    let doc = doc!{
        "user_id": activity.user_id,
        "activity": activity.activity,
        "datetime": activity.datetime
    };

    let coll = conn.collection("activities");
    coll.insert_one(doc, None).map(drop)
}

fn list_activities(conn: &Database)  -> Result<Vec<Activity>, Error> {
    conn.collection("activities").find(None, None)?
        .try_fold(Vec::new(), |mut vec, doc| {
            let doc = doc?;
            let activity : Activity = bson::from_bson(bson::Bson::Document(doc))?;
            vec.push(activity);
            Ok(vec)
        })
}

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
                .help("Sets an address of db connection")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name(CMD_ADD).about("add user to the table")
            .arg(Arg::with_name("USER_ID")
                .help("Sets the id of a user")
                .required(true)
                .index(1))
            .arg(Arg::with_name("ACTIVITY")
                .help("Sets the activity of a user")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name(CMD_LIST).about("print activities list of users"))
        .get_matches();

    let addr = matches.value_of("database")
        .unwrap_or("mongodb://localhost:27017/admin");
    let url = Url::parse(addr)?;

    let opts = ConnectionOptionsBuilder::new()
        .with_host(url.host_str().unwrap_or("localhost"))
        .with_port(url.port().unwrap_or(27017))
        .with_db(&url.path()[1..])
        .build();

    let manager = MongodbConnectionManager::new(opts);

    let pool = Pool::builder()
        .max_size(4)
        .build(manager)?;

    let conn = pool.get()?;

    match  matches.subcommand() {
        (CMD_ADD, Some(matches)) => {
            let user_id = matches.value_of("USER_ID").unwrap().to_owned();
            let activity = matches.value_of("ACTIVITY").unwrap().to_owned();
            let activity = Activity {
                user_id,
                activity,
                datetime: Utc::now().to_string()
            };

            add_activity(&conn, activity)?
        },
        (CMD_LIST, _) => {
            let list = list_activities(&conn)?;
            for item in list {
                println!("User: {:20} - Activity : {:20} - Datetime: {:20}", item.user_id, item.activity, item.datetime);
            }
        },
        _ => { matches.usage(); }
    }

    Ok(())
}
