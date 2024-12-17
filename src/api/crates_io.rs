use reqwest::Response;
use serde_json::Value;

use crate::{api, error};

/// get the latest version of a package from GitHub
pub fn get_latest(args: api::ApiArgs) -> api::ReleaseFuture {
    Box::pin(async move {
        let url = format!("https://crates.io/api/v1/crates/{}/versions", args.args[0]);

        let result = request(url, &args).await?;
        let json: Value = result.json().await?;
        let json = json.get("versions").unwrap();

        for version in json.as_array().unwrap() {
            let ver = version.get("num").unwrap().to_string().replace("\"", "");

            if ver.contains("-") {
                continue;
            }

            return Ok(api::Release {
                name: ver.clone(),
                tag: None,
                url: format!("https://crates.io/crates/{}/{}", args.args[0], ver),
            });
        }

        Err(error::Error::NoVersion(args.args[0].clone()))
    })
}

async fn request(url: String, args: &api::ApiArgs) -> error::Result<Response> {
    let headers = api::setup_headers();
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
        args: vec![package],
        api_key: String::new(),
    };

    assert!(get_latest(args).await.is_ok());
}
