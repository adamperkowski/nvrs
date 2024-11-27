# Contributing

Thank you for considering contributing to [nvrs](https://github.com/adamperkowski/nvrs) ❤️

If, while viewing the code, you find any parts unclear or unexplained, please [open an issue](https://github.com/adamperkowski/nvrs/issues/new/choose) with a documentation request.

Note that we have a [Code of Conduct](./CODE_OF_CONDUCT.md). Please follow it in all your interactions with the project.

## Workflow

1. Fork this repository and create your branch from `main`.

2. Clone your forked repository.

```sh
git clone https://github.com/adamperkowski/nvrs && cd nvrs
# OR
git clone git@github.com:adamperkowski/nvrs && cd nvrs
```

3. Make sure that you have [Rust](https://rust-lang.org) installed and build the project.

```sh
cargo build
```

4. Start committing your changes. Follow the [conventional commit specification](https://conventionalcommits.org).

5. Make sure [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy) don't complain about your changes.

6. If needed, update the changelog. Make sure that you have [git-cliff](https://github.com/orhun/git-cliff) installed.

```sh
git-cliff > CHANGELOG.md
git add CHANGELOG.md
git commit -m "changelog for $(git rev-parse --short HEAD)"
```

## Submitting Changes

1. Ensure that you updated the documentation and filled the [Pull Request template](./.github/PULL_REQUEST_TEMPLATE.md) according to the changes you made.

2. Push the changes and [open a Pull Request](https://github.com/adamperkowski/nvrs/pull/new).

3. Wait for review from the project's [CODE OWNER](./.github/CODEOWNERS). Update the Pull Request accordingly.

4. Once the Pull Request is approved, it will be merged into the main branch.

<!--         The above guidelines were inspired by git-cliff's CONTRIBUTING.md          -->
<!--          Copyright (c) 2021-2024 Orhun Parmaksız, git-cliff contributors           -->
<!-- https://github.com/orhun/git-cliff/commit/2e65a72bb044bad94f2568c491e4907f92331a56 -->
