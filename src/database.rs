// // // Backend Database Manager
// use rusqlite::Connection;
// use std::path;

// pub fn connect(database:Option<&str>) -> Connection {
//   match database {
//     Some(database) => {
//       match  path::Path::new(database).exists() {
//           true => return Connection::open(database).unwrap(),
//           false => return Connection::open(":memory:").unwrap(),
//       }
//     },
    
//     None => return Connection::open(":memory:").unwrap(),
//   }
// }

// pub fn query(database:Option<&str>, query: &str) -> usize {
//   let conn = connect(database);
//   let exe = conn.execute(query, []);

//   return exe.unwrap()
// }

//     // let queries = vec![
//     //     "CREATE TABLE users (name TEXT, age INTEGER);",
//     //     "INSERT INTO users VALUES ('Alice', 42);",
//     //     "INSERT INTO users VALUES ('Bob', 69);",
//     // ];
//     // let schema_query = "SELECT name FROM sqlite_master;";

//     // let conn = data::connect(None);
//     // let _setup = conn.execute(queries[0], []).unwrap();
    
//     // let sq = conn.prepare(&schema_query);

//     // match sq {
//     //     Ok(sq) => {
//     //         let x = sq.query_map([], |r| {
//     //             Ok(r.get(0))
//     //         })
//     //         println!("{:#?}", sq);
//     //     },
//     //     Err(e) => println!("{:#?}", e),
//     // }
