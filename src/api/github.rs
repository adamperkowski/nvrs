use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION};

use crate::api;

#[derive(serde::Deserialize)]
struct GitHubResponse {
    tag_name: String,
    html_url: String,
}

pub fn get_latest(args: api::ApiArgs) -> api::ReleaseFuture {
    Box::pin(async move {
        let url = format!(
            "https://api.github.com/repos/{}/releases/latest",
            args.args[0]
        );
        let mut headers = api::setup_headers();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_static("2022-11-28"),
        );
        if !args.api_key.is_empty() {
            let bearer = format!("Bearer {}", args.api_key);
            headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer).unwrap());
        }
        let client = args.request_client;

        let result = client.get(url).headers(headers).send().await?;
        api::match_statuscode(&result, args.package)?;

        let json: GitHubResponse = result.json().await?;

        Ok(api::Release {
            name: json.tag_name.clone(),
            tag: Some(json.tag_name),
            url: json.html_url,
        })
    })
}
