// // JSON File Handling
// use std::fs::write;

// pub fn write_json(table: &str, data: Vec<Repository>) {
//     let filename = table.replace(" ", "-");
//     let json_data = serde_json::to_string_pretty(&data)
//         .expect("failed to parse json");
    
//     write(format!("database/{}.json", filename), json_data
//         ).expect("failed to write json file");
    
//     println!("Data written to : {}.json", filename);
        
// }