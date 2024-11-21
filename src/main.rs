use nvrs::*;

#[tokio::main]
async fn main() -> error::Result<()> {
    let core = init().await?;

    if core.cli.cmp {
        compare(core).await;
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
        verfiles
    })
}

async fn compare(core: Core) {
    let config = core.config;
    let (oldver, newver) = core.verfiles;

    for package in newver.data.packages {

    }
}
