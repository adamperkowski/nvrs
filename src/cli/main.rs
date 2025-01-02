use colored::Colorize;

use nvrs::*;

mod args;

#[tokio::main]
async fn main() -> error::Result<()> {
    match init().await {
        Ok(core) => {
            let res = if core.1.cmp {
                compare(core.0).await
            } else if core.1.take.is_some() {
                take(core.0, core.1.take).await
            } else if core.1.nuke.is_some() {
                nuke(core.0, core.1.nuke, core.1.no_fail).await
            } else {
                sync(core.0, core.1.no_fail).await
            };

            match res {
                Ok(_) => (),
                Err(e) => e.pretty(),
            }
        }
        Err(e) => e.pretty(),
    }

    Ok(())
}

async fn init() -> error::Result<(Core, args::Cli)> {
    let cli = args::get_args();
    let config = config::load(&cli.custom_config).await?;

    let verfiles = verfiles::load(&config.0.__config__).await?;
    let keyfile = keyfile::load(&config.0.__config__).await?;

    Ok((
        Core {
            config,
            verfiles,
            client: reqwest::Client::new(),
            keyfile,
        },
        cli,
    ))
}

async fn compare(core: Core) -> error::Result<()> {
    let (oldver, newver) = core.verfiles;

    for new_pkg in newver.data.data {
        if let Some(old_pkg) = oldver.data.data.iter().find(|p| p.0 == &new_pkg.0) {
            if old_pkg.1.version != new_pkg.1.version {
                println!(
                    "{} {} {} -> {}",
                    "*".white().on_black(),
                    new_pkg.0.blue(),
                    old_pkg.1.version.red(),
                    new_pkg.1.version.green()
                );
            }
        } else {
            println!(
                "{} {} {} -> {}",
                "*".white().on_black(),
                new_pkg.0.blue(),
                "NONE".red(),
                new_pkg.1.version.green()
            );
        }
    }

    Ok(())
}

async fn take(core: Core, take_names: Option<Vec<String>>) -> error::Result<()> {
    let names = take_names.unwrap();
    let config = core.config;
    let (mut oldver, newver) = core.verfiles;

    let packages_to_update = if names.contains(&"ALL".to_string()) {
        newver.data.data.keys().cloned().collect()
    } else {
        names
    };

    for package_name in packages_to_update {
        if let Some(new_pkg) = newver.data.data.get(&package_name) {
            if let Some(old_pkg) = oldver.data.data.get_mut(&package_name) {
                if old_pkg.version != new_pkg.version {
                    println!(
                        "{} {} {} -> {}",
                        "+".white().on_black(),
                        package_name.blue(),
                        old_pkg.version.red(),
                        new_pkg.version.green()
                    );

                    let pkg = new_pkg.to_owned();
                    old_pkg.version = pkg.version;
                    old_pkg.gitref = pkg.gitref;
                    old_pkg.url = pkg.url;
                }
            } else {
                println!(
                    "{} {} {} -> {}",
                    "+".white().on_black(),
                    package_name.blue(),
                    "NONE".red(),
                    new_pkg.version.green()
                );
                oldver.data.data.insert(package_name, new_pkg.to_owned());
            }
        } else {
            return Err(error::Error::PkgNotInNewver(package_name));
        }
    }

    verfiles::save(&oldver, true, &config.0.__config__).await
}

async fn nuke(core: Core, nuke_names: Option<Vec<String>>, no_fail: bool) -> error::Result<()> {
    let names = nuke_names.unwrap();
    let mut config_content = core.config.0;
    let (mut oldver, mut newver) = core.verfiles;

    for package_name in names {
        if config_content.packages.contains_key(&package_name) {
            config_content.packages.remove(&package_name);
        } else if no_fail {
            error::Error::PkgNotInConfig(package_name.clone()).pretty();
        } else {
            return Err(error::Error::PkgNotInConfig(package_name));
        }
        newver.data.data.remove(&package_name);
        oldver.data.data.remove(&package_name);
    }

    verfiles::save(&newver, false, &config_content.__config__).await?;
    verfiles::save(&oldver, true, &config_content.__config__).await?;
    config::save(&config_content, core.config.1).await?;

    Ok(())
}

async fn sync(core: Core, no_fail: bool) -> error::Result<()> {
    let config = core.config.0;
    let (_, mut newver) = core.verfiles;

    let tasks: Vec<_> = config
        .packages
        .clone()
        .into_iter()
        .map(|pkg| tokio::spawn(run_source(pkg, core.client.clone(), core.keyfile.clone())))
        .collect();

    let mut results = futures::future::join_all(tasks).await;

    for package in config.packages {
        match results.remove(0).unwrap() {
            Ok(release) => {
                let gitref: String;
                let tag = if let Some(t) = release.tag.clone() {
                    gitref = format!("refs/tags/{}", t);
                    release.tag.unwrap().replacen(&package.1.prefix, "", 1)
                } else {
                    gitref = String::new();
                    release.name
                };

                if let Some(new_pkg) = newver.data.data.iter_mut().find(|p| p.0 == &package.0) {
                    if new_pkg.1.version != tag {
                        println!(
                            "{} {} {} -> {}",
                            "|".white().on_black(),
                            package.0.blue(),
                            new_pkg.1.version.red(),
                            tag.green()
                        );

                        new_pkg.1.version = tag;
                        new_pkg.1.gitref = gitref;
                        new_pkg.1.url = release.url;
                    }
                } else {
                    println!(
                        "{} {} {} -> {}",
                        "|".white().on_black(),
                        package.0.blue(),
                        "NONE".red(),
                        tag.green()
                    );

                    newver.data.data.insert(
                        package.0,
                        verfiles::VerPackage {
                            version: tag,
                            gitref,
                            url: release.url,
                        },
                    );
                }
            }
            Err(e) => {
                if !no_fail {
                    return Err(e);
                } else {
                    e.pretty();
                }
            }
        };
    }

    verfiles::save(&newver, false, &config.__config__).await
}

#[tokio::test]
async fn core_initializing() {
    assert!(init().await.is_ok())
}
