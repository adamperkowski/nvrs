# configuration
nvrs relies on a TOML configuration file ([example](https://github.com/adamperkowski/nvrs/blob/main/nvrs.toml)) containing basic settings, such as `oldver`, `newver` & `keyfile` paths, as well as [package entries](/package-entries.md). supported config paths:
- `./nvrs.toml`
- `$NVRS_CONFIG_DIR/nvrs.toml`
- `$XDG_CONFIG_HOME/nvrs/nvrs.toml` (`$HOME/.config/nvrs/nvrs.toml` if the variable is not set)
- `$HOME/.config/nvrs.toml`
- custom paths set with `nvrs --config`
