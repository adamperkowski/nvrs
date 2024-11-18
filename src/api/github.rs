use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    StatusCode,
};
use serde_json::Value;

pub fn get_latest(package: String, args: Vec<String>, key: String) -> crate::api::ReleaseFuture {
    Box::pin(async move {
        let url = if args[1] == "true" {
            format!("https://api.github.com/repos/{}/tags", args[0])
        } else {
            format!("https://api.github.com/repos/{}/releases/latest", args[0])
        };

        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("nvrs"));
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_static("2022-11-28"),
        );
        if !key.is_empty() {
            let bearer = format!("Bearer {}", key);
            headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer).unwrap());
        }
        let client = reqwest::Client::new();

        let result = client.get(url).headers(headers).send().await.unwrap();

        match result.status() {
            StatusCode::OK => (),
            StatusCode::FORBIDDEN => {
                crate::custom_error(
                    "GET request returned 430: ",
                    format!("{}\nwe might be getting rate-limited here", package),
                    "",
                );
                return None;
            }
            status => {
                crate::custom_error(
                    "GET request didn't return 200: ",
                    format!("{}\n{}", package, status),
                    "",
                );
                return None;
            }
        }

        if args[1] == "true" {
            let json: Value = result.json().await.unwrap();
            let name = json
                .get(0)
                .unwrap()
                .get("name")
                .unwrap()
                .to_string()
                .replace("\"", "");

            Some(crate::api::Release {
                tag_name: name.clone(),
                html_url: format!("https://github.com/{}/releases/tag/{}", args[0], name),
            })
        } else {
            Some(result.json().await.unwrap())
        }
    })
}
