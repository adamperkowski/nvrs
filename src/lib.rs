// nvrs - fast new version checker for software releases 🦀

// thiserror implementation
pub mod error;

// communication with sources
pub mod api;

// command-line arguments
pub mod cli;

// operations on configuration files
pub mod config;

// operations on version files
pub mod verfiles;

// example "core" vars structure
pub struct Core {
    pub cli: cli::Cli,
    pub config: config::Config,
    pub verfiles: (verfiles::Verfile, verfiles::Verfile),
}
