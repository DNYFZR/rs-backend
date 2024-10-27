// API Handline
use reqwest;
use serde_json;
use urlencoding::encode;

// Github API Data Structs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubRepos {
    total_count: Option<i32>,
    incomplete_results: Option<bool>,
    items: Vec<Repository>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    id: i32,
    node_id: String,
    name: String,
    full_name: String,
    private: bool,
    owner: Owner,
    html_url: String,
    description: Option<String>,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    teams_url: String,
    hooks_url: String,
    issue_events_url: String,
    events_url: String,
    assignees_url: String,
    branches_url: String,
    tags_url: String,
    blobs_url: String,
    git_tags_url: String,
    git_refs_url: String,
    trees_url: String,
    statuses_url: String,
    languages_url: String,
    stargazers_url: String,
    contributors_url: String,
    subscribers_url: String,
    subscription_url: String,
    commits_url: String,
    git_commits_url: String,
    comments_url: String,
    issue_comment_url: String,
    contents_url: String,
    compare_url: String,
    merges_url: String,
    archive_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    milestones_url: String,
    notifications_url: String,
    labels_url: String,
    releases_url: String,
    deployments_url: String,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    homepage: Option<String>,
    size: i32,
    stargazers_count: i32,
    watchers_count: i32,
    language: Option<String>,
    has_issues: bool,
    has_projects: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    forks_count: i32,
    mirror_url: Option<String>, // Make mirror_url optional
    archived: bool,
    disabled: bool,
    open_issues_count: i32,
    license: Option<License>, // Make license optional
    allow_forking: bool,
    is_template: bool,
    topics: Vec<String>,
    visibility: String,
    default_branch: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Owner {
    login: String,
    id: i32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    type_: Option<String>, // Use type_ to avoid conflict with Rust's type keyword
    site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct License {
    key: String,
    name: String,
    spdx_id: String,
    url: Option<String>, // Make url optional
    node_id: String,
}

pub async fn get(query: &str) -> Result<GitHubRepos, Box<dyn std::error::Error>>{
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
            return Ok(res);
        },

        Err(e) => {
            return Err(Box::new(e));
        }, 
    }
}
