inputs: {
  config,
  lib,
  pkgs,
  ...
}: let
  inherit
    (lib)
    mkOption
    mkEnableOption
    mkPackageOption
    types
    mkIf
    ;
  cfg = config.services.coffee-labeler;
in {
  options.services = {
    coffee-labeler = {
      enable = mkEnableOption "coffee-labeler";
      package = mkPackageOption pkgs "coffee-labeler" {};
      address = mkOption {
        description = ''
          Address to listen on
        '';
        type = types.str;
        default = "127.0.0.1";
        example = "[::1]";
      };
      port = mkOption {
        description = ''
          Port to listen on
        '';
        type = types.port;
        default = 3333;
      };
      printer-address = mkOption {
        description = ''
          Address of the label printer
        '';
        type = types.str;
        default = "192.168.178.36";
        example = "[::1]";
      };
      printer-port = mkOption {
        description = ''
          Port of the label printer
        '';
        type = types.port;
        default = 9100;
      };
    };
  };
  config = {
    nixpkgs.overlays = [
      inputs.self.overlays.default
    ];
    systemd.services = {
      coffee-labeler = mkIf cfg.enable {
        description = "coffee-labeler";
        after = ["network.target"];
        wantedBy = ["multi-user.target"];
        serviceConfig = {
          ExecStart = "${cfg.package}/bin/coffee-labeler";
          DynamicUser = true;
          Restart = "on-failure";
        };
        environment = {
          COFFEE_LABELER_ADDRESS = cfg.address;
          COFFEE_LABELER_PORT = toString cfg.port;
          COFFEE_LABELER_PRINTER_ADDRESS = cfg.printer-address;
          COFFEE_LABELER_PRINTER_PORT = toString cfg.printer-port;
        };
      };
    };
  };
}
