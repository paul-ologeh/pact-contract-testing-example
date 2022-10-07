use reqwest::Error;
use user_provider::{UserIn, UserOut};
use uuid::Uuid;

pub async fn get_user(base_url: String, id: Uuid) -> Result<UserOut, Error> {
    let client = reqwest::Client::new();
    let url = format!("{}user/{}", base_url, id);

    client.get(url).send().await?.json().await
}

pub async fn create_user(base_url: String, user: UserIn) -> Result<UserOut, Error> {
    let client = reqwest::Client::new();
    let url = format!("{}user", base_url);

    client.post(url).json(&user).send().await?.json().await
}
