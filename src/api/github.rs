use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT},
    StatusCode,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub html_url: String,
}

pub async fn get_latest(repo: String) -> Release {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("nvrs"));
    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_static("2022-11-28"),
    );
    let client = reqwest::Client::new();

    let result = client.get(url).headers(headers).send().await.unwrap();

    match result.status() {
        StatusCode::OK => (),
        status => {
            crate::custom_error("GET request didn't return 200", format!("\n{}", status));
        }
    }

    result.json().await.unwrap()
}
