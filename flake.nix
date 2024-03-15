{
  description = "Coffee labeler";
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = {
    self,
    devshell,
    crane,
    flake-utils,
    nixpkgs,
    pre-commit-hooks,
    rust-overlay,
  } @ inputs:
    {
      nixosModules.coffee-labeler = import ./nix/module.nix inputs;
      nixosModules.default = self.nixosModules.coffee-labeler;
      overlays.default = _final: prev: {
        inherit (self.packages.${prev.stdenv.hostPlatform.system}) coffee-labeler;
      };
    }
    // flake-utils.lib.eachDefaultSystem (localSystem: let
      pkgs = import nixpkgs {
        inherit localSystem;
        overlays = [
          devshell.overlays.default
          rust-overlay.overlays.default
        ];
      };
      inherit (pkgs) lib;

      rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      commonArgs = {
        src = lib.cleanSourceWith {
          src = ./.;
          filter = path: type: (craneLib.filterCargoSources path type) || (lib.hasSuffix ".proto" (builtins.baseNameOf path));
        };
        buildInputs = lib.optionals pkgs.stdenv.isDarwin [
          # Additional darwin specific inputs can be set here
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          pkgs.darwin.apple_sdk.frameworks.CoreFoundation
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.libiconv
        ];
      };

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      # Build the actual package
      package = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
          nativeBuildInputs = [pkgs.makeWrapper];
          postInstall = ''
            wrapProgram $out/bin/coffee-labeler \
              --prefix PATH : ${lib.makeBinPath [pkgs.chromium]}
          '';
        });
    in {
      checks =
        {
          coffee-labeler = package;

          coffee-labeler-clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          coffee-labeler-doc = craneLib.cargoDoc (commonArgs
            // {
              inherit cargoArtifacts;
            });

          coffee-labeler-fmt = craneLib.cargoFmt {
            inherit (commonArgs) src;
          };
        }
        // {
          pre-commit = pre-commit-hooks.lib.${localSystem}.run {
            src = ./.;
            hooks = {
              alejandra.enable = true;
              # cargo-check.enable = true;
              # rustfmt.enable = true;
              statix.enable = true;
            };
          };
        };

      packages.default = package;
      packages.coffee-labeler = package;

      devShells.default = pkgs.devshell.mkShell {
        name = "coffee-labeler";
        imports = [
          "${devshell}/extra/language/rust.nix"
        ];
        language.rust.enableDefaultToolchain = false;

        commands = [
          {
            package = pkgs.alejandra;
            help = "Format nix code";
          }
          {
            package = pkgs.statix;
            help = "Lint nix code";
          }
          {
            package = pkgs.deadnix;
            help = "Find unused expressions in nix code";
          }
        ];
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
            value = "192.168.178.36";
          }
          {
            name = "COFFEE_LABELER_PRINTER_PORT";
            value = 9100;
          }
        ];
        devshell.startup.pre-commit.text = self.checks.${localSystem}.pre-commit.shellHook;
        packages =
          commonArgs.buildInputs
          ++ (with pkgs; [
            rustToolchain
            nil
            probe-run
            rust-analyzer
            cargo-flamegraph
            cargo-watch
            cargo-modules
          ]);
      };

      formatter = pkgs.alejandra;
    });
}
