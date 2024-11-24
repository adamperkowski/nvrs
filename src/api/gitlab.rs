use crate::api;
use reqwest::header::HeaderValue;

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
        let url = format!(
            "https://{}/api/v4/projects/{}/releases/permalink/latest",
            host,
            args.args[0].replace("/", "%2F")
        );
        let mut headers = api::setup_headers();
        if let Some(key) = args.api_key {
            headers.insert("PRIVATE-TOKEN", HeaderValue::from_str(&key).unwrap());
        };
        let client = args.request_client;

        let result = client.get(url).headers(headers).send().await?;
        api::match_statuscode(&result, args.package)?;

        let json: GitLabResponse = result.json().await?;

        Ok(api::Release {
            name: json.tag_name.clone(),
            tag: Some(json.tag_name),
            url: format!("https://{}{}", host, json.tag_path),
        })
    })
}
