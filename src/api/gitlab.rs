use crate::{api, error};
use reqwest::{header::HeaderValue, Response};
use serde_json::Value;

#[derive(serde::Deserialize)]
struct GitLabResponse {
    tag_name: String,
    tag_path: String,
}

pub fn get_latest(args: api::ApiArgs) -> api::ReleaseFuture {
    Box::pin(async move {
        let host = if !args.args[1].is_empty() {
            &args.args[1]
        } else {
            "gitlab.com"
        };
        let repo_url = format!(
            "https://{}/api/v4/projects/{}",
            host,
            args.args[0].replace("/", "%2F")
        );

        if args.use_max_tag.is_some_and(|x| x) {
            let url = format!("{}/repository/tags", repo_url);
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
                url: format!("https://{}/{}/-/tags/{}", host, args.args[0], max_tag),
            })
        } else {
            let url = format!("{}/releases/permalink/latest", repo_url);
            let result = request(url, &args).await?;
            let json: GitLabResponse = result.json().await?;

            Ok(api::Release {
                name: json.tag_name.clone(),
                tag: Some(json.tag_name),
                url: format!("https://{}{}", host, json.tag_path),
            })
        }
    })
}

async fn request(url: String, args: &api::ApiArgs) -> error::Result<Response> {
    let mut headers = api::setup_headers();
    if !args.api_key.is_empty() {
        headers.insert(
            "PRIVATE-TOKEN",
            HeaderValue::from_str(&args.api_key).unwrap(),
        );
    };
    let client = &args.request_client;

    let result = client.get(url).headers(headers).send().await?;
    api::match_statuscode(&result.status(), args.package.clone())?;

    Ok(result)
}

#[tokio::test]
async fn request_test() {
    let package = "mkinitcpio".to_string();
    let args = api::ApiArgs {
        request_client: reqwest::Client::new(),
        package: package.clone(),
        use_max_tag: None,
        args: vec![
            format!("archlinux/{0}/{0}", package),
            "gitlab.archlinux.org".to_string(),
        ],
        api_key: String::new(),
    };

    assert!(get_latest(args).await.is_ok());
}
