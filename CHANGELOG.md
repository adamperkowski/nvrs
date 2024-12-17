# Changelog

All notable changes to nvrs will be documented in this file.

## [upstream]

### 🚀 Features

- `--list-sources` command ([442c06f](https://github.com/adamperkowski/nvrs/commit/442c06f0e56f4adcc0c2ad44d042997cb088a930))
- (*sources*) add regex ([#13](https://github.com/adamperkowski/nvrs/issues/13)) ([fa12ce9](https://github.com/adamperkowski/nvrs/commit/fa12ce9691adbdcf51990eb8416aedf4fdc7d36b))
- (*sources*) add crates.io ([78294ff](https://github.com/adamperkowski/nvrs/commit/78294ff44f26ce79df0c7c7c2511517627fc4a31))

### 🐛 Bug Fixes

- incorrect `--compare` colors ([ef1f78f](https://github.com/adamperkowski/nvrs/commit/ef1f78fed76f883986734b7e3220b2f56508a5f0))
- (*io*) not shutting down file streams after writing ([a8a42fd](https://github.com/adamperkowski/nvrs/commit/a8a42fdf03bfde7aeee563fbd6f9d7af832bc70e))

### 🛠️ Refactoring

- (*verfiles, config*) saving & loading improvements ([81d7efd](https://github.com/adamperkowski/nvrs/commit/81d7efd24b9b425f59bec1cdbb588bc25ed433cb))
- split features & binaries ([4acd98e](https://github.com/adamperkowski/nvrs/commit/4acd98e5d2c5df0845c6bd512cb40f705da9361e))

### 📚 Documentation

- more details & improvements ([9f02405](https://github.com/adamperkowski/nvrs/commit/9f02405339c3520340899313365f0de2fb3d65c5))
- (*README*) update cargo install instructions ([88f0fdc](https://github.com/adamperkowski/nvrs/commit/88f0fdce435c50df44c3ae2cfd5d1087df4376fc))
- improve the wording ([c47df3d](https://github.com/adamperkowski/nvrs/commit/c47df3dbe342bacc79ba2099f17365f07555cdd3))

### ⚙️ Miscellaneous Tasks

- (*repo*) fix dependabot ([432b10f](https://github.com/adamperkowski/nvrs/commit/432b10f32199ecd7a33c2d9643a5e1f512db862c))

### Other (unconventional)

- _ ([42da383](https://github.com/adamperkowski/nvrs/commit/42da3838cc80f5929fb592cfa8468f166238c32f))
- add the grind compliant badge ([3372b3e](https://github.com/adamperkowski/nvrs/commit/3372b3e7505d4854b10a63851743d54acffe1f7b))

## [0.1.5] - 2024-11-29

### 🚀 Features

- `use_max_tag` functionality ([8431412](https://github.com/adamperkowski/nvrs/commit/843141248520b7a784cae15c0571cd23e68d277e))

### 🐛 Bug Fixes

- (*ui*) `sync` errors displayed twice when no `--no-fail` ([8d7e341](https://github.com/adamperkowski/nvrs/commit/8d7e3413e258ac1b1a38256de10f02d8f078d68d))
- `Package` default() & new() features ([779c1ef](https://github.com/adamperkowski/nvrs/commit/779c1ef7e21c4ed6c31a524797a3f94c8678a3d5))

### 🛠️ Refactoring

- (*features*) remove `http` ([712bcea](https://github.com/adamperkowski/nvrs/commit/712bceae2626838af664df10dd967cb4a2819ab8))

### 📚 Documentation

- (*README*) add installation & usage instructions ([cd0bd72](https://github.com/adamperkowski/nvrs/commit/cd0bd7269f35ccb559f81abfda62c69ae06bce79))

### 🧪 Testing

- `Package` default(), new() & tests ([ca96da8](https://github.com/adamperkowski/nvrs/commit/ca96da8381da62cea1b01fd1f1d0363b7e5d1f9b))
- add benchmarking ([#5](https://github.com/adamperkowski/nvrs/issues/5)) ([3e1ef8d](https://github.com/adamperkowski/nvrs/commit/3e1ef8dbc6030073523d332652b3200016591071))

## [0.1.4] - 2024-11-25

### 🚀 Features

- (*take*) `ALL` functionality ([0ee83eb](https://github.com/adamperkowski/nvrs/commit/0ee83eb785f939780c8e07920c1f98a8a258d158))

### 🐛 Bug Fixes

- (*verfile*) allow missing gitref & url ([b93216d](https://github.com/adamperkowski/nvrs/commit/b93216d5146a672897e11938668e05cfa859cfac))
- `--nuke` not working ([15b75d9](https://github.com/adamperkowski/nvrs/commit/15b75d99667a4c52d0d9b093704aa02ca4d35e3e))

### 🛠️ Refactoring

- (*codebase*) [**breaking**] move internal logic to `lib` ([#4](https://github.com/adamperkowski/nvrs/issues/4)) ([c0021f0](https://github.com/adamperkowski/nvrs/commit/c0021f0a4e02791802fba9ba6bca5486f825ee4e))

### 📚 Documentation

- (*git-cliff*) add `UI/UX` ([42727ad](https://github.com/adamperkowski/nvrs/commit/42727ad6bd020ecee06e93017e7e5b68851c01d3))
- (*config*) fix the package name (alpm -> mkinitcpio) ([1327516](https://github.com/adamperkowski/nvrs/commit/132751692941f5e1e2cce188d545f3ee421dad46))
- better banner ([a4718b6](https://github.com/adamperkowski/nvrs/commit/a4718b60505d26c2e262b70d77160b475b8f2348))
- (*dependabot*) change cargo commit message ([90d50ab](https://github.com/adamperkowski/nvrs/commit/90d50ab0fd6cd4964408796e2f75affeb539923b))
- 🚦 ([f2e22b6](https://github.com/adamperkowski/nvrs/commit/f2e22b6c8daece310080a8e32d183e0f6ef3e3f0))

### 🧩 UI/UX

- (*output*) print out `NONE` take information ([71cb36f](https://github.com/adamperkowski/nvrs/commit/71cb36f913035d484bf26d8a2c3430132ea176ba))

## [0.1.3] - 2024-11-18

### 🐛 Bug Fixes

- not updating newver refs & urls ([2d3c48e](https://github.com/adamperkowski/nvrs/commit/2d3c48e097beb569dae2d610f35aaec03614e835))
- saving empty strings ([4b7a48a](https://github.com/adamperkowski/nvrs/commit/4b7a48a49ad39e49e2d98f4b87c2d7eb387c843d))
- messed up packages order ([8e5d63a](https://github.com/adamperkowski/nvrs/commit/8e5d63ad97b66fa5783d579241ba82c7499a47d2))

### 📚 Documentation

- (*config*) add a keyfile ([6f4ec82](https://github.com/adamperkowski/nvrs/commit/6f4ec82d58e099feca69b757c864da7a932a84fa))
- (*manpage*) add `--no-fail` ([86b7b6e](https://github.com/adamperkowski/nvrs/commit/86b7b6ef02dc2be981ffd8b5597c56b0dd70f27d))
- CONTRIBUTING.md, CODE_OF_CONDUCT.md, README badges, PULL_REQUEST_TEMPLATE.md ([ece2f01](https://github.com/adamperkowski/nvrs/commit/ece2f01ac8934c0bf4ae1eca4d895896ed1ac336))
- (*git-cliff*) disable github usernames ([889d365](https://github.com/adamperkowski/nvrs/commit/889d365dbd53f861ff4aa85633599a0996b09326))

### ⚡ Performance

- I. AM. SPEED. ([6933f8c](https://github.com/adamperkowski/nvrs/commit/6933f8ccafa0ae4f195e65921541e5eeb12b05fb))

### ⚙️ Miscellaneous Tasks

- fix invalid `is_empty_string` type ([1e8ae8f](https://github.com/adamperkowski/nvrs/commit/1e8ae8f44c24a1a70c7424c979ab4b654000f29c))

## [0.1.2] - 2024-11-17

### 🚀 Features

- (*sources*) multiple sources + AUR ([8322ada](https://github.com/adamperkowski/nvrs/commit/8322adaac003dd9210bd291399b275eb5daaf673))
- `--no-fail` ([4db55bc](https://github.com/adamperkowski/nvrs/commit/4db55bcd2ff55c7c137f511ce40999b6afe2b3f7))
- gitlab support ([4c46d82](https://github.com/adamperkowski/nvrs/commit/4c46d828bd55196a1ea094b5a2f9d037948b87e1))
- [**breaking**] keyfiles ([8ae2c27](https://github.com/adamperkowski/nvrs/commit/8ae2c27b71cb3fabd66623a13b9a8241c56deaad))

### 🐛 Bug Fixes

- (*aur*) quotes ([b1b3fcf](https://github.com/adamperkowski/nvrs/commit/b1b3fcf64c7591dc87ba201ecf54a4029fbd1960))
- (*aur*) quotes again ([9c2fedf](https://github.com/adamperkowski/nvrs/commit/9c2fedf1d7d4bbebe5a1ca9d8bfd204daee4283f))

### 📚 Documentation

- (*readme*) add `sources` ([0823f46](https://github.com/adamperkowski/nvrs/commit/0823f46aea5e19f31605360849bfeec2389c51af))

### ⚙️ Miscellaneous Tasks

- more `cargo` metadata ([6b6ebd6](https://github.com/adamperkowski/nvrs/commit/6b6ebd680f49d22c053360f7b542ba074e3eb2b1))
- (*main.rs*) collapse the `latest` `else if` statement ([3cdb71d](https://github.com/adamperkowski/nvrs/commit/3cdb71dc8e1759eb6a3309d5fe45dfe95663fc02))
- (*gitignore*) add `keyfile.toml` ([602b91f](https://github.com/adamperkowski/nvrs/commit/602b91fba795ec8916bbdb4131d4a89975b157bf))

## [0.1.1] - 2024-11-17

### 🚀 Features

- `--nuke` functionality + some minor fixes ([6949ec0](https://github.com/adamperkowski/nvrs/commit/6949ec0c36c3634dafd0123b5ee7cbd4c092e0c9))
- add `--version` & about ([50f2bc2](https://github.com/adamperkowski/nvrs/commit/50f2bc246aa32b0f50fb3aa55580c56559c5ee64))

### 🐛 Bug Fixes

- (*ui*) wrong --cmp output characters ([3cad4c1](https://github.com/adamperkowski/nvrs/commit/3cad4c1dd94f54c176d894e32c4f7ef384c6d8dd))
- (*config*) make `prefix` optional ([7b942cc](https://github.com/adamperkowski/nvrs/commit/7b942cc6b9f7c5ac551837e7f53425df34ccb3a9))

### 📚 Documentation

- add a banner & move `speed` to `features` ([752fc15](https://github.com/adamperkowski/nvrs/commit/752fc158b118de603a9f2a9f31a0c320fb3cf78a))
- add a manpage ([073c98f](https://github.com/adamperkowski/nvrs/commit/073c98ff097283fae09742c77bb98358d706bb22))
- some `git-cliff` improvements ([270c0e6](https://github.com/adamperkowski/nvrs/commit/270c0e6b6e729a349b61a512def02433d3675cc9))
- more `git-cliff` improvements ([83ae70f](https://github.com/adamperkowski/nvrs/commit/83ae70fd0e2820158a56a86a05aa6f619ae6b141))

### ⚡ Performance

- drastically decrease bin size & increase performance ([460f9d9](https://github.com/adamperkowski/nvrs/commit/460f9d9bbe6928d34948ecb3eec7fd0c6c4b7ba4))

### Other (unconventional)

- change the `--take` character ([0aace9d](https://github.com/adamperkowski/nvrs/commit/0aace9de0f2c3f26eda4de9491a3454929398102))

## [0.1.0] - 2024-11-16

### 🐛 Bug Fixes

- (*hot*) a typo in `custom_error` ([4844515](https://github.com/adamperkowski/nvrs/commit/48445157be6b3ae9ca97d6c79f25b20529e30fd7))

### 🛠️ Refactoring

- (*custom_error*) improve newline control ([#1](https://github.com/adamperkowski/nvrs/issues/1)) ([05faaca](https://github.com/adamperkowski/nvrs/commit/05faaca79dd1306a818864ab80ae028a0217dd1e))

### ⚙️ Miscellaneous Tasks

- GitHub stuff ([eda40d8](https://github.com/adamperkowski/nvrs/commit/eda40d8d68c4c13d24ad2b9b0acd217c02ee889e))
- run git-cliff on schedule ([c18f152](https://github.com/adamperkowski/nvrs/commit/c18f15256d041c17f1a47e6310c08ce23fc286f2))
- exclude `CHANGELOG.md` from `typos` ([bbdd835](https://github.com/adamperkowski/nvrs/commit/bbdd83543aa49be2ca690e767d42d5572e3ee2a8))

### Other (unconventional)

- init ([4ca8ba6](https://github.com/adamperkowski/nvrs/commit/4ca8ba6f390d668e8d13caa0214f97c09115d4c3))
- set up workflows ([86933da](https://github.com/adamperkowski/nvrs/commit/86933da3817c26fa3caa6a84bb3ecf4c4d2cae2a))
- rebranding ([b927a53](https://github.com/adamperkowski/nvrs/commit/b927a536fddbde155979ef03ef0b800906ef777b))
- cli args ([97cca62](https://github.com/adamperkowski/nvrs/commit/97cca6211308b3eef82f16e8289527e7490f10a4))
- config ([b03dc12](https://github.com/adamperkowski/nvrs/commit/b03dc12e3686f0ef5e21f43731189a771d08d475))
- github api ([9c92e24](https://github.com/adamperkowski/nvrs/commit/9c92e24d3a2a82eaaf84f3b37ce342a8b88181cd))
- better cli ([683ffd7](https://github.com/adamperkowski/nvrs/commit/683ffd77f6fc03067b9929ee4c50f3c8600e75ff))
- custom configs ([51b78ba](https://github.com/adamperkowski/nvrs/commit/51b78baf83eb9f1fb2190974a2668263d1ce2e6c))
- verfiles + updating ([345f8fd](https://github.com/adamperkowski/nvrs/commit/345f8fda053074c150e7595e611b8d44dd603786))
- new entry saving ([62075ec](https://github.com/adamperkowski/nvrs/commit/62075ecdb5d4666b6b3fec6e02c42913f544c75c))
- compare & take ([18d538f](https://github.com/adamperkowski/nvrs/commit/18d538f738be4060fb65388cb822f09c8e00aebf))

<sub>generated by [git-cliff](https://github.com/orhun/git-cliff) :)</sub>
