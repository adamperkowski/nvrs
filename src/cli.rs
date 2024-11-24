use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};

const COPYRIGHT_TEXT: &str =
    "Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:\n
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.";

#[derive(Clone, Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(
        short = 'c',
        long,
        help = "Compare newver with oldver and display differences as updates"
    )]
    pub cmp: bool,

    #[arg(
        short = 't',
        long,
        value_name = "packages",
        help = "List of packages to update automatically, separated by a comma",
        value_delimiter = ','
    )]
    pub take: Option<Vec<String>>,

    #[arg(
        short = 'n',
        long,
        value_name = "packages",
        help = "List of packages to delete from the config",
        value_delimiter = ','
    )]
    pub nuke: Option<Vec<String>>,

    #[arg(
        long = "config",
        value_name = "path",
        help = "Override path to the config file"
    )]
    pub custom_config: Option<String>,

    #[arg(long, help = "Don't exit on recoverable errors")]
    pub no_fail: bool,

    #[arg(long, help = "Display copyright information")]
    copyright: bool,
}

pub fn get_args() -> Cli {
    let cli = Cli::parse();

    if cli.copyright {
        let current_year = 1970
            + (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs()
                / (365 * 24 * 60 * 60));

        println!(
            "Copyright (c) {} Adam Perkowski\n{}",
            current_year, COPYRIGHT_TEXT
        );
    }

    cli
}
