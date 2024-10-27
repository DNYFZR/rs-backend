// Application Backend 
mod api;
mod database;

use serde_json;
use std::fs;

#[tokio::main]
async fn main() {
    let url = "https://api.github.com/search/repositories?q=rust&sort=stars&order=desc&per_page=10&page=1";
    let raw_data = api::get_data(url).await.expect("failed to call API");
    
    // Test write out
    let json_data = serde_json::to_string_pretty(&raw_data).expect("failed to parse json");
    fs::write("database/test.json", json_data).expect("failed to write json file");


   println!("{:#?}", raw_data);
}


