use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    StatusCode,
};

pub fn get_latest(package: String, _: Vec<String>, _: String) -> crate::api::ReleaseFuture {
    Box::pin(async move {
        let url = format!("https://aur.archlinux.org/rpc/v5/info/{}", package);
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("nvrs"));
        let client = reqwest::Client::new();

        let result = client.get(url).headers(headers).send().await.unwrap();

        match result.status() {
            StatusCode::OK => (),
            status => {
                crate::custom_error("GET request didn't return 200", format!("\n{}", status), "");
                return None;
            }
        }

        let json: serde_json::Value = result.json().await.unwrap();
        let first_result = json.get("results").unwrap().get(0).unwrap();

        Some(crate::api::Release {
            tag_name: first_result
                .get("Version")
                .unwrap()
                .to_string()
                .split('-')
                .next()
                .unwrap_or("")
                .replace("\"", "")
                .to_string(),
            html_url: first_result
                .get("URL")
                .unwrap()
                .to_string()
                .replace("\"", ""),
        })
    })
}
