// Application Backend 
mod api;
mod database;
mod structs;

#[tokio::main]
async fn main() {
    // Program input
    let user_search:Vec<_> = std::env::args().collect();

    if user_search.len() > 1 {
        let table = "api_data";
        let search_term = user_search[1].clone();
        let name = search_term.replace(" ", "-");
        println!("Searching for : {:#?}", search_term);

        // Process API call
        let data = api::get(&search_term)
            .await
            .expect("failed to call API");
        
        // Update DB
        match database::update(&table, &name, data) {
            Ok(_) => println!("DB table {name} updated"),
            Err(e) => println!("Error : {e}"),
        }

        println!("|--- SUCCESS ---|");
    } 
    
    else {
        println!("Please provide a search term, using quotes if whitespace is required")
    }    
}


