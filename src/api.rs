// API Handline
use crate::structs::{ GitHubRepos, Repository};

use reqwest;
use serde_json;
use urlencoding::encode;


pub async fn get(query: &str) -> Result<Vec<Repository>, Box<dyn std::error::Error>>{
    // Create url
    let query = encode(query);
    let url = format!("https://api.github.com/search/repositories?q={}&sort=stars&order=desc&per_page=10&page=1", query);
    
    // Create header
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        reqwest::header::USER_AGENT, 
        reqwest::header::HeaderValue::from_static("MyCustomUserAgent/1.0")
    );

    // Make API call
    let client = reqwest::Client::new();
    let req = client 
        .get(&url)
        .headers(headers)
        .send()
        .await;

    // Process response
    match req {
        Ok(req) => {
            let res_raw = req.text().await.expect("failed to get response text");
            let res: GitHubRepos = serde_json::from_str(&res_raw).expect("Error parsing response text");
            return Ok(res.items);
        },

        Err(e) => {
            return Err(Box::new(e));
        }, 
    }
}

#[cfg(test)]
mod api_tests {
    use super::get;

    #[tokio::test]
    async fn test_get() {
        let test = get("rust").await;

        match test {
            Ok(res) => println!("success: {:#?}", res[0].name),
            Err(e) => println!("error : {e}"),
        }
    }
}