pub mod api;
pub mod config;
pub mod verfiles;

pub use crate::config::{Config, ConfigTable, Keyfile, Package};
pub use crate::verfiles::{Data, Package as VerPackage, Verfile};

use colored::Colorize;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref MSG_NOEXIT: Mutex<bool> = Mutex::new(false);
}

pub async fn run_source(
    package: (String, Package),
    keyfile: Option<Keyfile>,
) -> Option<api::Release> {
    let source = package.1.source.clone();
    if let Some(api_used) = api::API_LIST.iter().find(|a| a.name == source) {
        let api_key = if let Some(k) = keyfile {
            k.get_api_key(source)
        } else {
            String::new()
        };

        Some(
            (api_used.func)(
                package.0,
                package.1.get_api_arg(api_used.name).unwrap(),
                api_key,
            )
            .await?,
        )
    } else {
        custom_error("api not found: ", source, "");
        None
    }
}

pub fn custom_error(message: &'static str, message_ext: String, override_exit: &str) {
    println!("! {}{}", message.red(), message_ext.replace('\n', "\n  "));

    if override_exit != "noexit" && !*MSG_NOEXIT.lock().unwrap() {
        std::process::exit(1);
    }
}
