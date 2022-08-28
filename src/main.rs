extern crate serde_json;
use reqwest::Url;
use serde_json::Value;
use twitter_v2::authorization::BearerToken;

async fn twitter_api(route: &str) -> Result<Value, Box<reqwest::Error>> {
    let base = Url::parse("https://api.twitter.com/2/").unwrap();

    let url = base.join(route).unwrap();

    let client = reqwest::Client::new();

    let res = client.get(url.as_str()).send().await?;

    let body = res.text().await?;

    let v: Value = serde_json::from_str(body.as_str()).unwrap();

    Ok(v)
}

async fn get_user_by_username(username: &str) -> Result<Value, Box<reqwest::Error>> {
    let route = format!("/users/by/username/{}", username);
    return twitter_api(&route).await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bearer_token = std::env::var("TWITTER_BEARER_TOKEN").unwrap();
    let auth = BearerToken::new(bearer_token);

    let data = get_user_by_username("zachkrall").await.unwrap();

    println!("{}", data);

    Ok(())
}
