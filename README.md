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
check the [release notes](https://github.com/adamperkowski/nvrs/releases) and [configuration docs](#configuration) for compatibility updates and instructions.

### Speed
<img align='right' src='https://media1.tenor.com/m/mMWXOkCEndoAAAAC/ka-chow-lightning-mcqueen.gif' alt='ka-chow' width=80 height=45>

| command       | time per **updated** package | details                                                |
|---------------|------------------------------|--------------------------------------------------------|
| `nvrs`        | ~ 0.03s                      | **API requests included**<br>depends on internet speed |
| `nvrs --cmp`  | ~ 0.0008s                    | depends on disk speed                                  |
| `nvrs --take` | ~ 0.001s                     | depends on disk speed                                  |

### Sources
- `aur`
- `cratesio`
- `gitea`
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

see the [installation guide](https://nvrs.adamperkowski.dev/installation.html) for instructions on how to install nvrs.

## Usage
nvrs relies on a configuration file. see [configuration](https://nvrs.adamperkowski.dev/configuration.html). 

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

for all available commands, options and flags, see `nvrs --help` and the [manual page](/man/nvrs.1).

## Documentation
full documentation can be found at [nvrs.adamperkowski.dev](https://nvrs.adamperkowski.dev).
nvrs library documentation can be found at [docs.rs/nvrs](https://docs.rs/nvrs/latest/nvrs)

## Contributing
if you want to contribute to the project, please read the [Contributing Guidelines](/CONTRIBUTING.md) before doing so.

if you find any parts of the code or the documentation unclear, or have any suggestions, feel free to [open an issue](https://github.com/adamperkowski/nvrs/issues/new/choose) or a [pull request](https://github.com/adamperkowski/nvrs/pull/new).

## Credits
- [依云](https://github.com/lilydjwg) | the original [nvchecker](https://github.com/lilydjwg/nvchecker)
- [orhun](https://github.com/orhun) | the idea

<div align='center'>

<sub align='center'>Copyright (c) 2025 Adam Perkowski<br>see [LICENSE](/LICENSE)</sub>

</div>
