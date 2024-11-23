use clap::Parser;
use colored::Colorize;
use config::Keyfile;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

mod api;
pub mod config;
mod verfiles;

lazy_static::lazy_static! {
    static ref MSG_NOEXIT: Mutex<bool> = Mutex::new(false);
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(
        short = 'c',
        long,
        help = "Compare newver with oldver and display differences as updates"
    )]
    cmp: bool,

    #[arg(
        short = 't',
        long,
        value_name = "packages",
        help = "List of packages to update automatically, separated by a comma",
        value_delimiter = ','
    )]
    take: Option<Vec<String>>,

    #[arg(
        short = 'n',
        long,
        value_name = "packages",
        help = "List of packages to delete from the config",
        value_delimiter = ','
    )]
    nuke: Option<Vec<String>>,

    #[arg(
        long = "config",
        value_name = "path",
        help = "Override path to the config file"
    )]
    custom_config: Option<String>,

    #[arg(long, help = "Don't exit the program on recoverable errors")]
    no_fail: bool,

    #[arg(long, help = "Display copyright information")]
    copyright: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.no_fail {
        *MSG_NOEXIT.lock().unwrap() = true;
    }

    if cli.copyright {
        let current_year = 1970
            + (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs()
                / (365 * 24 * 60 * 60));

        println!(
            "Copyright (c) {} Adam Perkowski\n
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:\n
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.",
            current_year
        );
    } else if cli.cmp {
        let (config_content, _, _) = config::load(cli.custom_config);
        let (oldver, newver) = verfiles::load(config_content.__config__.clone()).unwrap();

        for package in newver.data.data {
            if let Some(pkg) = oldver.data.data.iter().find(|p| p.0 == &package.0) {
                if pkg.1.version != package.1.version {
                    println!(
                        "* {} {} -> {}",
                        package.0.blue(),
                        pkg.1.version.red(),
                        package.1.version.green()
                    );
                }
            } else {
                println!(
                    "* {} {} -> {}",
                    package.0.blue(),
                    "NONE".red(),
                    package.1.version.green()
                );
            }
        }
    } else if cli.take.is_some() {
        let names = cli.take.unwrap();
        let (config_content, _, _) = config::load(cli.custom_config);
        let (mut oldver, newver) = verfiles::load(config_content.__config__.clone()).unwrap();

        for package_name in names {
            if let Some(package) = newver.data.data.iter().find(|p| p.0 == &package_name) {
                if let Some(pkg) = oldver.data.data.iter_mut().find(|p| p.0 == &package_name) {
                    if pkg.1.version != package.1.version {
                        println!(
                            "+ {} {} -> {}",
                            package.0.blue(),
                            pkg.1.version.red(),
                            package.1.version.green()
                        );
                    } else {
                        println!(
                            "+ {} {} -> {}",
                            package.0.blue(),
                            pkg.1.version,
                            package.1.version
                        );
                    }
                    pkg.1.version = package.1.version.clone();
                    pkg.1.gitref = package.1.gitref.clone();
                    pkg.1.url = package.1.url.clone();
                } else {
                    println!(
                        "+ {} {} -> {}",
                        package.0.blue(),
                        "NONE".red(),
                        package.1.version.green()
                    );
                    oldver.data.data.insert(package_name, package.1.clone());
                }
            } else {
                custom_error("package not in newver: ", package_name, "noexit");
            }
        }

        verfiles::save(oldver, true, config_content.__config__).unwrap();
    } else if cli.nuke.is_some() {
        let names = cli.nuke.unwrap();
        let (mut config_content, config_content_path, _) = config::load(cli.custom_config);
        let (mut oldver, mut newver) = verfiles::load(config_content.__config__.clone()).unwrap();

        for package_name in names {
            if config_content.packages.contains_key(&package_name) {
                config_content.packages.remove(&package_name);
            } else {
                custom_error("package not in config: ", package_name.clone(), "noexit");
            }
            newver.data.data.remove(&package_name);
            oldver.data.data.remove(&package_name);
        }

        verfiles::save(newver, false, config_content.__config__.clone()).unwrap();
        verfiles::save(oldver, true, config_content.__config__.clone()).unwrap();
        config::save(config_content, config_content_path).unwrap();
    } else {
        let (config_content, _, keyfile) = config::load(cli.custom_config);
        let (_, mut newver) = verfiles::load(config_content.__config__.clone()).unwrap();

        let tasks: Vec<_> = config_content
            .packages
            .clone()
            .into_iter()
            .map(|pkg| tokio::spawn(run_source(pkg, keyfile.clone())))
            .collect();

        let mut results = futures::future::join_all(tasks).await;

        for package in config_content.packages {
            let release = results.remove(0).unwrap().unwrap();
            let tag = release.tag_name.replacen(&package.1.prefix, "", 1);

            if let Some(pkg) = newver.data.data.iter_mut().find(|p| p.0 == &package.0) {
                if pkg.1.version != tag {
                    println!(
                        "| {} {} -> {}",
                        package.0.blue(),
                        pkg.1.version.red(),
                        tag.green()
                    );
                    pkg.1.version = tag;
                    pkg.1.gitref = format!("refs/tags/{}", release.tag_name);
                    pkg.1.url = release.html_url;
                }
            } else {
                println!("| {} {} -> {}", package.0.blue(), "NONE".red(), tag.green());
                newver.data.data.insert(
                    package.0,
                    verfiles::Package {
                        version: tag,
                        gitref: format!("refs/tags/{}", release.tag_name),
                        url: release.html_url,
                    },
                );
            }
        }

        verfiles::save(newver, false, config_content.__config__).unwrap();
    }
}

pub async fn run_source(
    package: (String, config::Package),
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
    println!("! {}{}", message.red(), message_ext.replace("\n", "\n  "));

    if override_exit != "noexit" && !*MSG_NOEXIT.lock().unwrap() {
        std::process::exit(1);
    }
}
