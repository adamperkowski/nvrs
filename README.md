<div align='center'>

# nvrs
🚦 fast new version checker for software releases 🦀

[![Grind Compliant](https://img.shields.io/badge/Grind-Compliant-blue?style=for-the-badge&labelColor=%23a8127d&color=%23336795)](https://github.com/The-Grindhouse/guidelines)<br>
![Build Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/nvrs/rust.yml?style=for-the-badge&labelColor=%23a8127d&color=%23336795) [![docs.rs](https://img.shields.io/docsrs/nvrs?style=for-the-badge&labelColor=%23a8127d&color=%23336795)](#documentation)<br>
[![GitHub Contributors](https://img.shields.io/github/contributors-anon/adamperkowski/nvrs?style=for-the-badge&labelColor=%23a8127d&color=%23336795)](https://github.com/adamperkowski/nvrs/graphs/contributors) ![GitHub Repo Size](https://img.shields.io/github/repo-size/adamperkowski/nvrs?style=for-the-badge&labelColor=%23a8127d&color=%23336795) ![Repo Created At](https://img.shields.io/github/created-at/adamperkowski/nvrs?style=for-the-badge&labelColor=%23a8127d&color=%23336795)

![banner](/banner.webp)

</div>

## Features
### [nvchecker](https://github.com/lilydjwg/nvchecker) compatibility
check the [release notes](https://github.com/adamperkowski/nvrs/releases) for compatibility updates

### TUI

### Speed
<img align='right' src='https://media1.tenor.com/m/mMWXOkCEndoAAAAC/ka-chow-lightning-mcqueen.gif' alt='ka-chow' width=80 height=45>

| command       | time per **updated** package | details                                                |
|---------------|------------------------------|--------------------------------------------------------|
| `nvrs`        | ~ 0.03s                      | **API requests included**<br>depends on internet speed |
| `nvrs --cmp`  | ~ 0.0008s                    | depends on disk speed                                  |
| `nvrs --take` | ~ 0.001s                     | depends on disk speed                                  |

### Sources
- `aur`
- `github`
- `gitlab` (with custom hosts)
- `website` (regex)

### QOL improvements
- `ALL` argument for the `--take` command
- `--no-fail` flag to prevent exiting on recoverable errors
- `--nuke` command to delete packages from all files
- `--list-sources` command to list all available sources

## Installation
<a href="https://repology.org/project/nvrs/versions"><img align="right" src="https://repology.org/badge/vertical-allrepos/nvrs.svg" alt="Packaging status"></a>

<details>
<summary>Arch Linux</summary>

[nvrs](https://aur.archlinux.org/packages/nvrs) is available as a package in the [AUR](https://aur.archlinux.org).<br>
you can install it with your preferred [AUR helper](https://wiki.archlinux.org/title/AUR_helpers), example:

```sh
paru -S nvrs
```

or manually:

```sh
git clone https://aur.archlinux.org/nvrs.git
cd nvrs
makepkg -si
```

</details>

<details>
<summary>Cargo</summary>

[nvrs](https://crates.io/crates/nvrs) can be installed via [Cargo](https://doc.rust-lang.org/cargo) with:

```sh
cargo install nvrs --all-features
```

note that crates installed using `cargo install` require manual updating with `cargo install --force`.

</details>

<details>
<summary>Manual</summary>

1. Download the latest binary from [GitHub's release page](https://github.com/adamperkowski/nvrs/releases/latest)
2. Allow execution
```sh
chmod +x nvrs
```
3. Move the file to a directory in `$PATH` (using `/usr/bin` as an example)
```sh
sudo mv nvrs /usr/bin/nvrs
```

</details>

## Usage
nvrs relies on a configuration file. see [configuration](#configuration). 

<img align='center' src='https://vhs.charm.sh/vhs-7j0ZLSJUnq5W8xwqjK14W4.gif' alt='Packaging status'>

the core commands are:
- `nvrs` - fetches latest versions of defined packages
- `nvrs --cmp` - compares newver with oldver and displays differences
- `nvrs --take` - automatically updates oldver. takes in a comma-separated list of package names (`ALL` for all packages)
- `nvrs --nuke` - deletes packages from all files. takes in a comma-separated list of names (yes, just like a hitman)
- the `--no-fail` flag - as the name suggests, specifying this will make nvrs not exit on recoverable errors

### Example usage
```sh
# download the example configuration file
curl -L 'https://github.com/adamperkowski/nvrs/raw/main/nvrs.toml' -o nvrs.toml

# fetch latest package versions (should return `NONE -> version` for all packages)
nvrs --no-fail

# compare them to latest known versions (should also return `NONE -> version`)
nvrs -c

# update the known versions
nvrs -t ALL
```

## Configuration
nvrs relies on a configuration file ([example](/nvrs.toml)) containing basic settings, such as `oldver`, `newver` & `keyfile` paths, as well as [package entries](#package-entries). supported config paths:
- `$XDG_CONFIG_HOME/nvrs.toml` (`~/.config/nvrs.toml` if the variable is not set)
- `./nvrs.toml`
- custom paths set with `nvrs --config`

### `__config__` table
this configures the behavior of nvrs. see the [example config](/nvrs.toml#L7-L10).

available fields:

| name      | description                                                     | type   | required |
|-----------|-----------------------------------------------------------------|--------|----------|
| `oldver`  | path to the `oldver` file                                       | string | ✔️       |
| `newver`  | path to the `newver` file                                       | string | ✔️       |
| `keyfile` | path to a keyfile (see [keyfile structure](#keyfile-structure)) | string | ❌       |

### Package entries

[example](/nvrs.toml#L12-L15)

package entries are custom entries in the main config file. they contain values such as:

| name          | description                                                               | type   | required | custom |
|---------------|---------------------------------------------------------------------------|--------|----------|--------|
| `source`      | see [sources](#sources)                                                   | string | ✔️       | ❌     |
| source name   | the "target". eg. repo path for `github`                                  | string | ✔️       | ✔️     |
| `host`        | domain name the source is hosted on                                       | string | ❌       | ❌     |
| `prefix`      | the prefix used in releases / tags<br>example: `v` for tags like `v0.1.0` | string | ❌       | ❌     |
| `use_max_tag` | use max git tag instead of the latest release                             | bool   | ❌       | ❌     |
| `url`         | url to check for source type `regex`                                      | string | ❌       | ❌     |
| `regex`       | regex to search url for source type `regex`                               | bool   | ❌       | ❌     |

### Keyfile structure
this file contains API keys for various [sources](#sources). example can be found [here](/n_keyfile.toml).

```toml
[keys]
github = "your_secret_github_api_key_that_you_shouldnt_push_to_a_public_nor_a_private_remote_repo_because_there_will_definitely_be_serious_consequences_sooner_or_later_if_you_do_trust_me_just_dont"
gitlab = "remember_to_replace_the_example_values_here_here_with_your_actual_keys_otherwise_it_wont_work_but_dont_push_keyfiles_to_remote_repos"
```

<sub align='center'>"<i>I think that example value is not long enough</i>" - orhun</sub>

## Documentation
the nvrs library documentation can be found at [docs.rs/nvrs](https://docs.rs/nvrs/latest/nvrs)

## Credits
- [依云](https://github.com/lilydjwg) | the original [nvchecker](https://github.com/lilydjwg/nvchecker)
- [orhun](https://github.com/orhun) | the idea

<div align='center'>

<sub align='center'>Copyright (c) 2024 Adam Perkowski<br>see [LICENSE](/LICENSE)</sub>

</div>
