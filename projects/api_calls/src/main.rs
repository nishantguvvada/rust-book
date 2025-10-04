use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
    owner = "rust-lang-nursery",
    repo = "rust-cookbook"
    );
    println!("{request_url}");

    let client = reqwest::Client::new();
    let response = client
    .get(&request_url)
    .header(USER_AGENT, "rust web-api-client demo")
    .send()
    .await?;

    let users: Vec<User> = response.json().await?; // Even though you don't call serde_json::from_str() directly, reqwest uses this trait under the hood when you call .json() on a response.
    println!("{:?}", users);
    Ok(())
}

// .json() is a method provided by reqwest that automatically reads the body and deserializes it into the type you specify.
// The type Vec<User> tells reqwest:
// “I expect this JSON to be an array of objects matching the User struct.”
// Under the hood, reqwest uses serde_json::from_str() for you — that’s why we imported Deserialize but didn’t call serde_json ourselves.
// If the JSON matches Vec<User>, you get a fully typed Rust vector. If not, you get an error.

// let users: Vec<User> = response.json().await?;
// The above line is equivalent to:
// let body_text = response.text().await?;
// let users: Vec<User> = serde_json::from_str(&body_text)?;
