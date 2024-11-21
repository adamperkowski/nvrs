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

pub async fn get_latest(args: api::ApiArgs) -> error::Result<api::Release> {
    let url = format!("https://aur.archlinux.org/rpc/v5/info/{}", args.package);
    let client = args.request_client;

    let result = client.get(url).headers(api::setup_headers()).send().await?;

    api::match_statuscode(&result); // FIXME

    let json: AURResponse = result.json().await?;

    if let Some(first) = json.results.first() {
        let version = first.Version.split_once('-').unwrap();

        Ok(api::Release {
            name: version.0.to_string(),
            tag: None,
            url: None,
        })
    } else {
        Err(error::Error::NoVersion)
    }
}
