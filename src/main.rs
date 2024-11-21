use colored::Colorize;
use nvrs::*;

#[tokio::main]
async fn main() -> error::Result<()> {
    let core = init().await?;

    if core.cli.cmp {
        compare(core).await;
    } else if core.cli.take.is_some() {
        take(core).await?
    }

    let args = api::ApiArgs {
        request_client: reqwest::Client::new(),
        package: "tukai".to_string(),
        target: "".to_string(),
        host: None,
        api_key: None,
    };
    let r = api::aur::get_latest(args).await;
    match r {
        Ok(_) => println!("{:#?}", r.ok()),
        Err(e) => eprintln!("FIXME error displaying\npackage here or something\n{}", e),
    };

    Ok(())
}

async fn init() -> error::Result<Core> {
    let cli = cli::get_args();
    let config = config::load(cli.clone().custom_config).await?;

    // TODO: this could be handled entirely within lib
    let verfiles = verfiles::load(config.0.__config__.clone()).await?;

    Ok(Core {
        cli,
        config: config.0,
        verfiles,
    })
}

async fn compare(core: Core) {
    let (oldver, newver) = core.verfiles;

    for new_pkg in newver.data.data {
        if let Some(old_pkg) = oldver.data.data.iter().find(|p| p.0 == &new_pkg.0) {
            if old_pkg.1.version != new_pkg.1.version {
                println!(
                    "* {} {} -> {}",
                    new_pkg.0.blue(),
                    old_pkg.1.version.red(),
                    new_pkg.1.version.blue()
                );
            }
        } else {
            println!(
                "* {} {} -> {}",
                new_pkg.0.blue(),
                "NONE".red(),
                new_pkg.1.version.green()
            );
        }
    }
}

async fn take(core: Core) -> error::Result<()> {
    let names = core.cli.take.unwrap();
    let config = core.config;
    let (mut oldver, newver) = core.verfiles;

    for package_name in names {
        if let Some(new_pkg) = newver.data.data.iter().find(|p| p.0 == &package_name) {
            if let Some(old_pkg) = oldver.data.data.iter_mut().find(|p| p.0 == &package_name) {
                if old_pkg.1.version != new_pkg.1.version {
                    println!(
                        "+ {} {} -> {}",
                        package_name.blue(),
                        old_pkg.1.version.red(),
                        new_pkg.1.version.green()
                    );
                } else {
                    println!(
                        "+ {} {} -> {}",
                        package_name.blue(),
                        old_pkg.1.version.red(),
                        new_pkg.1.version.green()
                    );
                }
                old_pkg.1.version = new_pkg.1.version.clone();
                old_pkg.1.gitref = new_pkg.1.gitref.clone();
                old_pkg.1.url = new_pkg.1.url.clone();
            } else {
                println!(
                    "+ {} {} -> {}",
                    package_name.blue(),
                    "NONE".red(),
                    new_pkg.1.version.green()
                );
                oldver.data.data.insert(package_name, new_pkg.1.clone());
            }
        } else {
            return Err(error::Error::PkgNotInNewver);
        }
    }

    Ok(verfiles::save(oldver, true, config.__config__).await?)
}
