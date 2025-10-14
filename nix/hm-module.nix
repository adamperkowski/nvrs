{
  lib,
  pkgs,
  config,
  ...
}:

let
  settingsFormat = pkgs.formats.toml { };
in
{
  meta.maintainers = with lib.maintainers; [ adamperkowski ];

  options.programs.nvrs = {
    enable = lib.mkEnableOption "fast new version checker for software releases";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ./default.nix { };
      description = "nvrs package";
    };

    settings = lib.mkOption {
      type = lib.types.submodule {
        freeformType = settingsFormat.type;
      };

      default = { };
      description = ''
        Configuration written to {file}`$XDG_CONFIG_HOME/nvrs/config.toml`

        See <https://nvrs.adamperkowski.dev/configuration.html> for details.
      '';
    };
  };

  config = lib.mkIf config.programs.nvrs.enable {
    home.packages = [ config.programs.nvrs.package ];

    xdg.configFile."nvrs/config.toml" = lib.mkIf (config.programs.nvrs.settings != { }) {
      source = settingsFormat.generate "nvrs-config.toml" config.programs.nvrs.settings;
    };
  };
}
