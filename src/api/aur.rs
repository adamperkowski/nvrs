use crate::{api, error};

#[derive(serde::Deserialize)]
struct AURResponse {
    results: Vec<AURResult>,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct AURResult {
    Version: String,
}

/// get the latest version of a package from the AUR
pub fn get_latest(args: api::ApiArgs) -> api::ReleaseFuture {
    Box::pin(async move {
        let url = format!("https://aur.archlinux.org/rpc/v5/info/{}", args.args[0]);
        let client = args.request_client;

        let result = client.get(url).headers(api::setup_headers()).send().await?;
        api::match_statuscode(&result.status(), args.package.clone())?;

        let json: AURResponse = result.json().await?;

        if let Some(first) = json.results.first() {
            let version = first.Version.split_once('-').unwrap();

            Ok(api::Release {
                name: version.0.to_string(),
                tag: None,
                url: String::new(),
            })
        } else {
            Err(error::Error::NoVersion(args.package))
        }
    })
}

#[tokio::test]
async fn request_test() {
    let package = "permitter".to_string();
    let args = api::ApiArgs {
        request_client: reqwest::Client::new(),
        package: package.clone(),
        use_max_tag: None,
        args: vec![package],
        api_key: String::new(),
    };

    assert!(get_latest(args).await.is_ok());
}
