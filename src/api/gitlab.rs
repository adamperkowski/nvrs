use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT},
    StatusCode,
};
use serde_json::Value;

pub fn get_latest(package: String, args: Vec<String>, key: String) -> crate::api::ReleaseFuture {
    Box::pin(async move {
        let host = if !args[1].is_empty() {
            format!("https://{}/", args[1])
        } else {
            "https://gitlab.com/".to_string()
        };
        let prefix = format!("{}api/v4/projects/{}/", host, args[0].replace("/", "%2F"));

        let url = if args[2] == "true" {
            format!("{}{}", prefix, "tags")
        } else {
            format!("{}{}", prefix, "releases/permalink/latest")
        };

        let result = request(url, package, key).await.unwrap();
        let r_json: Value = result.json().await.unwrap();

        if args[2] == "true" {
            let name = r_json
                .get(0)
                .unwrap()
                .get("name")
                .unwrap()
                .to_string()
                .replace("\"", "");
            Some(crate::api::Release {
                tag_name: name.clone(),
                html_url: format!("{}/-/tags/{}", args[0].replace("/", "%2F"), name),
            })
        } else {
            Some(crate::api::Release {
                tag_name: r_json
                    .get("tag_name")
                    .unwrap()
                    .to_string()
                    .replace("\"", ""),
                html_url: r_json
                    .get("_links")
                    .unwrap()
                    .get("self")
                    .unwrap()
                    .to_string()
                    .replace("\"", ""),
            })
        }
    })
}

async fn request(url: String, package: String, key: String) -> Option<reqwest::Response> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("nvrs"));
    if !key.is_empty() {
        headers.insert(
            HeaderName::from_static("PRIVATE-TOKEN"),
            HeaderValue::from_str(key.as_str()).unwrap(),
        );
    }
    let client = reqwest::Client::new();

    let result = client.get(url).headers(headers).send().await.unwrap();

    match result.status() {
        StatusCode::OK => Some(result),
        StatusCode::FORBIDDEN => {
            crate::custom_error(
                "GET request returned 430: ",
                format!("{}\nwe might be getting rate-limited here", package),
                "",
            );
            None
        }
        status => {
            crate::custom_error(
                "GET request didn't return 200: ",
                format!("{}\n{}", package, status),
                "",
            );
            None
        }
    }
}
