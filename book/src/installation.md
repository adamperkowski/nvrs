# installation
<a href="https://repology.org/project/nvrs/versions"><img align="right" src="https://repology.org/badge/vertical-allrepos/nvrs.svg" alt="Packaging status"></a>

## Arch Linux
[nvrs](https://aur.archlinux.org/packages/nvrs) is available as a package in the [AUR](https://aur.archlinux.org).<br>
you can install it with your preferred [AUR helper](https://wiki.archlinux.org/title/AUR_helpers), example:

```sh
paru -S nvrs
```

or manually:

```
git clone https://aur.archlinux.org/nvrs.git
cd nvrs
makepkg -si
```

## Nix

### Flakes

There is a [flake](https://github.com/adamperkowski/nvrs/blob/main/flake.nix) available. <br>
You can run it directly with:

```bash
nix run github:adamperkowski/nvrs
```
or install it by adding the following to your `flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nvrs.url = "github:adamperkowski/nvrs";
  };

  outputs = { self, nixpkgs, nvrs }: {
    nixosConfigurations.example = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [{
        environment.systemPackages = [
          inputs.nvrs.packages.${pkgs.system}.default
        ];
      }];
    };
  };
}
```

### Home Manager

nvrs also provides a [Home Manager](https://nix-community.github.io/home-manager) module. <br>
You can use it in your `home.nix` like this:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nvrs.url = "github:adamperkowski/nvrs";
  };

  outputs = { self, nixpkgs, home-manager, nvrs }: {
    homeConfigurations."user@hostname" = home-manager.lib.homeManagerConfiguration {
      modules = [
        home-manager.homeManagerModules.default
        nvrs.homeManagerModules.default
        {
          programs.nvrs = {
            enable = true;
            settings = {
              julec = {
                source = "github";
                github = "julelang/julec";
                prefix = "jule";
              };
            };
          };
        }
      ];
    };
  };
}
```

## Cargo
[nvrs](https://crates.io/crates/nvrs) can be installed via [Cargo](https://doc.rust-lang.org/cargo) with:

```sh
cargo install nvrs --all-features
```

note that crates installed using `cargo install` require manual updating with `cargo install --force`.

## Manual
1. download the latest binary from [GitHub's release page](https://github.com/adamperkowski/nvrs/releases/latest)
2. allow execution
```sh
chmod +x nvrs
```
3. move the file to a directory in `$PATH` (using `/usr/bin` as an example)
```sh
sudo mv nvrs /usr/bin/nvrs
```
