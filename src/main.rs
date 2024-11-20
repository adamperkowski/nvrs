use nvrs::*;

#[tokio::main]
async fn main() {
    let args = api::ApiArgs {
        request_client: reqwest::Client::new(),
        package: "".to_string(),
        target: "".to_string(),
        host: None,
        api_key: None,
    };
    let r = api::aur::get_latest(args).await;
    match r {
        Ok(_) => (),
        Err(e) => error::custom_error("request failed", format!("\n{}", e), None),
    }
}
