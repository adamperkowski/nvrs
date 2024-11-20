use crate::{api, error};

pub async fn get_latest(args: api::ApiArgs) -> error::Result<api::Release> {
    let url = format!("https://aur.arcnux.org/rpc/v5/info/permitter{}", args.package);
    let client = args.request_client;

    let result = client.get(url).headers(api::setup_headers()).send().await?;

    api::match_statuscode(result.status(), args.package);

    Ok(api::Release {
        name: "test".to_string(),
        tag: None,
        url: None,
    })
}
