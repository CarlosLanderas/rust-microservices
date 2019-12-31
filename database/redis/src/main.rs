use clap::{
crate_authors, crate_description, crate_name, crate_version
};

use redis::{Commands, Connection, RedisError};
use r2d2_redis::RedisConnectionManager;
use std::collections::HashMap;

const SESSIONS : &str = "sessions";
const CMD_ADD : &str = "add";
const CMD_REMOVE: &str = "remove";
const CMD_LIST: &str = "list";

fn add_session(conn: &Connection, token: &str, uid: &str) -> Result<(), RedisError> {
    conn.hset(SESIONS, token, uid)
}

fn remove_session(conn: &Connection, token: &str) -> Result<(), RedisError> {
    conn.hdel(SESSIONS, token)
}

fn list_sessions(conn: &Connection) -> Result<HashMap<String,String>,RedisError> {
    conn.hgetall(SESSIONS)
}

fn main() {

}
