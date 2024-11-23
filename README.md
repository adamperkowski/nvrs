<div align='center'>

# nvrs
🚦 fast new version checker for software releases 🦀<br>
[nvchecker](https://github.com/lilydjwg/nvchecker) rewritten in Rust

![GitHub Contributors](https://img.shields.io/github/contributors-anon/adamperkowski/nvrs?style=for-the-badge&labelColor=%23a8127d&color=%23336795) ![GitHub Repo Size](https://img.shields.io/github/repo-size/adamperkowski/nvrs?style=for-the-badge&labelColor=%23a8127d&color=%23336795) ![Repo Created At](https://img.shields.io/github/created-at/adamperkowski/nvrs?style=for-the-badge&labelColor=%23a8127d&color=%23336795)

![banner](/banner.webp)

</div>

## WIP
nvrs is still a WIP<br>
new features & bugfixes are being pushed every day

you may encounter some issues. please consider [submitting feedback](https://github.com/adamperkowski/nvrs/issues/new/choose) if you do.

## Features
### Speed
<img src='https://media1.tenor.com/m/mMWXOkCEndoAAAAC/ka-chow-lightning-mcqueen.gif' alt='ka-chow' width=80 height=45>

| command       | time per **updated** package | details                                                |
|---------------|------------------------------|--------------------------------------------------------|
| `nvrs`        | ~ 0.04s                      | **API requests included**<br>depends on internet speed |
| `nvrs --cmp`  | ~ 0.0008s                    | depends on disk speed                                  |
| `nvrs --take` | ~ 0.001s                     | depends on disk speed                                  |

### Sources
**WIP**

- `aur`
- `github`
- `gitlab` (with custom hosts)

## Credits
- [依云](https://github.com/lilydjwg) | the original [nvchecker](https://github.com/lilydjwg/nvchecker)
- [orhun](https://github.com/orhun) | the idea

<div align='center'>

<sub align='center'>Copyright (c) 2024 Adam Perkowski<br>see [LICENSE](/LICENSE)</sub>

</div>
