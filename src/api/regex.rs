use crate::{api, error};
use regex::Regex;

/// get a version string from a webpage
pub fn get_latest(args: api::ApiArgs) -> api::ReleaseFuture {
    Box::pin(async move {
        let url = args.args[0].clone();
        let client = args.request_client;

        let result = client
            .get(&url)
            .headers(api::setup_headers())
            .send()
            .await?;
        api::match_statuscode(&result.status(), args.package.clone())?;

        let body = result.text().await?;

        let re = Regex::new(&args.args[1]).unwrap();
        if let Some(caps) = re.captures(&body) {
            Ok(api::Release {
                name: caps.get(1).unwrap().as_str().to_owned(),
                tag: None,
                url,
            })
        } else {
            Err(error::Error::NoVersion(args.package))
        }
    })
}

#[tokio::test]
async fn request_test() {
    let package = "rustrover".to_string();
    let args = api::ApiArgs {
        request_client: reqwest::Client::new(),
        package: package.clone(),
        use_max_tag: None,
        args: vec![
            "https://data.services.jetbrains.com/products?code=RR&release.type=release".to_string(),
            r"RustRover-([\d.]+).tar.gz".to_string(),
        ],
        api_key: String::new(),
    };

    assert!(crate::api::regex::get_latest(args).await.is_ok());
}
