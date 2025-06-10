use std::process::Command;

use crate::{api, error};

/// get the version from a custom shell command
pub fn get(args: api::ApiArgs) -> api::ReleaseFuture {
    Box::pin(async move {
        let result = Command::new("sh").arg("-c").arg(&args.args[0]).output()?;

        if !result.status.success() {
            return Err(error::Error::ShellCommandFailed(
                String::from_utf8_lossy(&result.stderr).trim().to_string(),
            ));
        }

        Ok(api::Release {
            name: String::from_utf8_lossy(&result.stdout).trim().to_string(),
            tag: None,
            url: String::new(),
        })
    })
}

#[tokio::test]
async fn shell_test() {
    let package = "testpkg".to_string();
    let args = api::ApiArgs {
        request_client: reqwest::Client::new(),
        package: package.clone(),
        use_max_tag: None,
        args: vec![format!("echo '0.1.0'")],
        api_key: String::new(),
    };

    let res = get(args).await;
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().name,
        "0.1.0".to_string(),
        "Shell command should return '0.1.0'"
    );
}
