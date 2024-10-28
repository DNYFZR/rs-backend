// Backend Data Storage Controller
use crate::structs::Repository;

use std::fs::write;
use rusqlite::{ Connection, Result };
use serde_json;

pub fn update(table:&str, data:Vec<Repository>) -> Result<(), serde_json::Error> {
    // Consts
    const DB: &str = "database/app.db"; 
    const CREATE_TABLE_QUERY: &str = "CREATE TABLE IF NOT EXISTS api_data (
            name TEXT PRIMARY KEY,
            data JSON
        )";

    // Open the database, falling back to in-memory if path is not usable
    let conn = match Connection::open(DB) {
        Ok(conn) => conn,
        Err(_) => Connection::open_in_memory().expect("failed to open in-memory DB"),
    };

    // Create the api_data table if it does not exist
    conn.execute(
        CREATE_TABLE_QUERY,
        [],
    ).expect("failed to create table");

    // Insert or update the data in the api_data table
    match serde_json::to_string(&data) {
        Ok(res) => {
            conn.execute(
                "INSERT OR REPLACE INTO api_data (name, data) VALUES (?1, ?2)",
                (table, res),
            ).expect(&format!("failed to update {}", table));
            return Ok(());
        },

        Err(e) => {
            println!("error : {e}");
            return Err(e);
        },
     }
}

fn write_json(table: &str, data: Vec<Repository>) {
    let filename = table.replace(" ", "-");
    let json_data = serde_json::to_string_pretty(&data)
        .expect("failed to parse json");
    
    write(format!("database/{}.json", filename), json_data
        ).expect("failed to write json file");
    
    println!("Data written to : {}.json", filename);
        
}