// Application Backend 
mod api;
mod database;

use serde_json;
use std::fs;

#[tokio::main]
async fn main() {
    // Program input
    let user_search:Vec<_> = std::env::args().collect();

    if user_search.len() > 1 {
        println!("Searching for : {:#?}", user_search[1]);

        // Process API call
        let raw_data = api::get(&user_search[1])
            .await
            .expect("failed to call API");
        
        let json_data = serde_json::to_string_pretty(&raw_data)
            .expect("failed to parse json");

        // Write out    
        fs::write(
            format!("database/{}.json", user_search[1].replace(" ", "-")),
            json_data
        ).expect("failed to write json file");
        
        println!("Output written to : {}.json", user_search[1].replace(" ", "-"));
        println!("|--- SUCCESS ---|");
    } 
    
    else {
        println!("Please provide a search term, using quotes if whitespace is required")
    }    
}


