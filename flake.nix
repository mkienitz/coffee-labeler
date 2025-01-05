{
  description = "Coffee Labeler";

  inputs = {
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      imports = [
        inputs.devshell.flakeModule
        inputs.flake-parts.flakeModules.easyOverlay
        inputs.nci.flakeModule
        inputs.pre-commit-hooks.flakeModule
        inputs.treefmt-nix.flakeModule
      ];

      flake =
        { config, ... }:
        {
          nixosModules.coffee-labeler = import ./nix/module.nix inputs;
          nixosModules.default = config.nixosModules.coffee-labeler;
        };

      perSystem =
        {
          pkgs,
          config,
          lib,
          ...
        }:
        let
          crateName = "coffee-labeler";
          projectName = crateName;
          crateOutput = config.nci.outputs.${crateName};
        in
        {
          nci = {
            projects.${projectName} = {
              path = ./.;
              numtideDevshell = "default";
            };
            crates.${crateName}.drvConfig.mkDerivation = {
              nativeBuildInputs = [ pkgs.makeWrapper ];
              postInstall = ''
                wrapProgram $out/bin/coffee-labeler \
                  --prefix PATH : ${lib.makeBinPath (lib.optionals pkgs.stdenv.isLinux [ pkgs.chromium ])}
              '';
            };
          };

          packages.default = crateOutput.packages.release;
          overlayAttrs.coffee-labeler = config.packages.default;

          devshells.default = {
            packages = [
              pkgs.nil
              pkgs.rust-analyzer
              pkgs.cargo-watch
            ];
            devshell.startup.pre-commit.text = config.pre-commit.installationScript;
            env = [
              {
                name = "COFFEE_LABELER_ADDRESS";
                value = "localhost";
              }
              {
                name = "COFFEE_LABELER_PORT";
                value = 3333;
              }
              {
                name = "COFFEE_LABELER_PRINTER_ADDRESS";
                value = "192.168.178.39";
              }
              {
                name = "COFFEE_LABELER_PRINTER_PORT";
                value = 9100;
              }
            ];
          };

          pre-commit.settings.hooks.treefmt.enable = true;

          treefmt = {
            projectRootFile = "flake.nix";
            programs = {
              deadnix.enable = true;
              statix.enable = true;
              nixfmt.enable = true;
              rustfmt.enable = true;
            };
          };
        };
    };
}
