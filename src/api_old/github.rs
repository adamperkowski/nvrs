use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    StatusCode,
};

pub fn get_latest(package: String, repo: Vec<String>, key: String) -> crate::api::ReleaseFuture {
    Box::pin(async move {
        let url = format!("https://api.github.com/repos/{}/releases/latest", repo[0]);
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

        Some(result.json().await.unwrap())
    })
}
