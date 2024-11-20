use serde::Deserialize;

#[cfg(feature = "aur")]
pub mod aur;
#[cfg(feature = "github")]
pub mod github;
#[cfg(feature = "gitlab")]
pub mod gitlab;

#[derive(Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub html_url: String,
}

pub type ReleaseFuture =
    std::pin::Pin<Box<dyn std::future::Future<Output = Option<Release>> + Send>>;

pub struct Api {
    pub name: &'static str,
    pub func: fn(String, Vec<String>, String) -> ReleaseFuture,
}

pub const API_LIST: &[Api] = &[
    #[cfg(feature = "aur")]
    Api {
        name: "aur",
        func: aur::get_latest,
    },
    #[cfg(feature = "github")]
    Api {
        name: "github",
        func: github::get_latest,
    },
    #[cfg(feature = "gitlab")]
    Api {
        name: "gitlab",
        func: gitlab::get_latest,
    },
];
