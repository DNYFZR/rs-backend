// Application Backend 
mod api;
mod database;
mod structs;

#[tokio::main]
async fn main() {
    // Program input
    let user_search:Vec<_> = std::env::args().collect();

    if user_search.len() > 1 {
        let search_term = user_search[1].clone();
        let table_name = search_term.replace(" ", "-");
        println!("Searching for : {:#?}", search_term);

        // Process API call
        let api_data = api::get(&search_term)
            .await
            .expect("failed to call API");

        // Update DB
        match database::update(&table_name, api_data) {
            Ok(_) => println!("DB table {table_name} updated"),
            Err(e) => println!("Error : {e}"),
        }

        println!("|--- SUCCESS ---|");
    } 
    
    else {
        println!("Please provide a search term, using quotes if whitespace is required")
    }    
}


