use crate::{api, error};
use reqwest::{header::HeaderValue, Response};

#[derive(serde::Deserialize)]
#[serde(transparent)]
struct GiteaTagResponse {
    tags: Vec<GiteaTag>,
}

#[derive(serde::Deserialize)]
struct GiteaTag {
    name: String,
    commit: GiteaCommit,
}

#[derive(serde::Deserialize)]
#[serde(transparent)]
struct GiteaCommitResponse {
    commits: Vec<GiteaCommit>,
}

#[derive(serde::Deserialize)]
struct GiteaCommit {
    sha: String,
    url: String,
    commit: Option<GiteaRepoCommit>,
}

#[derive(serde::Deserialize)]
struct GiteaRepoCommit {
    author: GiteaCommitAuthor,
}
#[derive(serde::Deserialize)]
struct GiteaCommitAuthor {
    date: String,
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
            let url = format!("{}/tags", repo_url);

            let result = request(url, &args).await?;
            let json: GiteaTagResponse = result.json().await?;

            Ok(api::Release {
                name: json.tags[0].name.clone(),
                tag: Some(json.tags[0].name.clone()),
                url: json.tags[0].commit.url.clone(),
            })
        } else {
            let url = format!("{}/commits", repo_url);
            let result = request(url, &args).await?;
            let json: GiteaCommitResponse = result.json().await?;

            Ok(api::Release {
                name: json
                    .commits
                    .first()
                    .unwrap()
                    .commit
                    .as_ref()
                    .unwrap()
                    .author
                    .date
                    .clone(),
                tag: Some(json.commits[0].sha.clone()),
                url: json.commits[0].url.clone(),
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
        use_max_tag: None,
        args: vec![package, "codeberg.org".to_string()],
        api_key: String::new(),
    };

    assert!(get_latest(args).await.is_ok());
}

#[tokio::test]
async fn request_test_max_tag() {
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
