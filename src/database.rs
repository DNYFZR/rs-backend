// // Backend Database Manager
use rusqlite::Connection;
use std::path;

fn connect(database:Option<&str>) -> Connection {
  match database {
    Some(database) => {
      match  path::Path::new(database).exists() {
          true => return Connection::open(database).unwrap(),
          false => return Connection::open(":memory:").unwrap(),
      }
    },
    
    None => return Connection::open(":memory:").unwrap(),
  }
}

pub fn query(database:Option<&str>, query: &str) -> usize {
  let conn = connect(database);
  let exe = conn.execute(query, []);

  return exe.unwrap()
}
