# configuration
nvrs relies on a TOML configuration file ([example](https://github.com/adamperkowski/nvrs/blob/main/nvrs.toml)) containing basic settings, such as `oldver`, `newver` & `keyfile` paths, as well as [package entries](/package-entries.md). supported config paths:
- `$XDG_CONFIG_HOME/nvrs.toml` (`~/.config/nvrs.toml` if the variable is not set)
- `./nvrs.toml`
- custom paths set with `nvrs --config`
