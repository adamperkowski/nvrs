# Changelog

All notable changes to nvrs will be documented in this file.

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

### ⚙️ Refactoring

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
