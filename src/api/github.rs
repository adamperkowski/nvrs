use reqwest::{
    header::{HeaderValue, ACCEPT, AUTHORIZATION},
    Response,
};
use serde_json::Value;

use crate::{api, error};

#[derive(serde::Deserialize)]
struct GitHubResponse {
    tag_name: String,
    html_url: String,
}

pub fn get_latest(args: api::ApiArgs) -> api::ReleaseFuture {
    Box::pin(async move {
        let repo_url = format!("https://api.github.com/repos/{}", args.args[0]);

        if args.use_max_tag.is_some_and(|x| x) {
            let url = format!("{}/tags", repo_url);
            let result = request(url, &args).await?;
            let json: Value = result.json().await?;

            let max_tag = json
                .get(0)
                .unwrap()
                .get("name")
                .unwrap()
                .to_string()
                .replace("\"", "");

            Ok(api::Release {
                name: max_tag.clone(),
                tag: Some(max_tag.clone()),
                url: format!(
                    "https://github.com/{}/releases/tag/{}",
                    args.args[0], max_tag
                ),
            })
        } else {
            let url = format!("{}/releases/latest", repo_url);
            let result = request(url, &args).await?;
            let json: GitHubResponse = result.json().await?;

            Ok(api::Release {
                name: json.tag_name.clone(),
                tag: Some(json.tag_name),
                url: json.html_url,
            })
        }
    })
}

async fn request(url: String, args: &api::ApiArgs) -> error::Result<Response> {
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
    let client = &args.request_client;

    let result = client.get(url).headers(headers).send().await?;
    api::match_statuscode(&result.status(), args.package.clone())?;

    Ok(result)
}

#[tokio::test]
async fn request_test() {
    let package = "nvrs".to_string();
    let args = api::ApiArgs {
        request_client: reqwest::Client::new(),
        package: package.clone(),
        use_max_tag: None,
        args: vec![format!("adamperkowski/{}", package)],
        api_key: String::new(),
    };

    assert!(get_latest(args).await.is_ok());
}
