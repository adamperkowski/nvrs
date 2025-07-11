use crate::{api, error};
use reqwest::{Response, header::HeaderValue};

#[derive(serde::Deserialize)]
#[serde(transparent)]
struct GiteaTagResponse {
    tags: Vec<GiteaTag>,
}

#[derive(serde::Deserialize)]
struct GiteaTag {
    name: String,
}

#[derive(serde::Deserialize)]
#[serde(transparent)]
struct GiteaReleaseResponse {
    releases: Vec<GiteaRelease>,
}

#[derive(serde::Deserialize)]
struct GiteaRelease {
    tag_name: String,
    html_url: String,
}

/// get the latest version of a package from Gitea
pub fn get_latest(args: api::ApiArgs) -> api::ReleaseFuture {
    Box::pin(async move {
        let host = if !args.args[1].is_empty() {
            &args.args[1]
        } else {
            "gitea.com"
        };
        let repo_url = format!("https://{}/api/v1/repos/{}", host, args.args[0]);

        if args.use_max_tag.is_some_and(|x| x) {
            let url = format!("{repo_url}/tags");

            let result = request(url, &args).await?;
            let json: &GiteaTag = &result.json::<GiteaTagResponse>().await?.tags[0];

            Ok(api::Release {
                name: json.name.clone(),
                tag: Some(json.name.clone()),
                url: format!("{}/releases/tag/{}", repo_url, json.name),
            })
        } else {
            let url = format!("{repo_url}/releases");
            let result = request(url, &args).await?;
            let json: &GiteaRelease = &result.json::<GiteaReleaseResponse>().await?.releases[0];

            let tag = json.tag_name.to_owned();

            Ok(api::Release {
                name: tag.clone(),
                tag: Some(tag),
                url: json.html_url.clone(),
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
    let package = "maandree/libkeccak".to_string();
    let args = api::ApiArgs {
        request_client: reqwest::Client::new(),
        package: package.clone(),
        use_max_tag: Some(true),
        args: vec![package, "codeberg.org".to_string()],
        api_key: String::new(),
    };

    assert!(get_latest(args).await.is_ok());
}
