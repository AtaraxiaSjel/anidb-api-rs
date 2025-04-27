{
  description = "Build a cargo workspace";

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    devenv.url = "github:cachix/devenv";
    devenv-root = {
      url = "file+file:///dev/null";
      flake = false;
    };
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      systems = [ "x86_64-linux" ];

      perSystem =
        {
          pkgs,
          system,
          lib,
          ...
        }:
        let
          rustToolchain = pkgs.fenix.stable.defaultToolchain;

          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
          src = craneLib.cleanCargoSource ./.;

          commonArgs = rec {
            inherit src;
            strictDeps = true;

            nativeBuildInputs = [
              pkgs.mold
              # pkgs.pkg-config
            ];

            buildInputs = [
              pkgs.openssl
            ];

            # LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          anidb-api = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
              inherit (craneLib.crateNameFromCargoToml { inherit src; }) pname version;
              doCheck = false;
            }
          );
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.fenix.overlays.default ];
          };

          checks = {
            inherit anidb-api;

            # Run clippy (and deny all warnings) on the workspace source.
            anidb-clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets -- --deny warnings";
              }
            );

            # Generate doc
            anidb-doc = craneLib.cargoDoc (
              commonArgs
              // {
                inherit cargoArtifacts;
              }
            );

            # Check formatting
            anidb-fmt = craneLib.cargoFmt {
              inherit src;
            };

            # Check formatting for TOML files
            anidb-toml-fmt = craneLib.taploFmt {
              src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
              taploExtraArgs = "--config ./taplo.toml";
            };

            # Audit dependencies
            anidb-audit = craneLib.cargoAudit {
              inherit src;
              advisory-db = inputs.advisory-db;
            };

            # Audit licenses
            anidb-deny = craneLib.cargoDeny {
              inherit src;
            };

            # Run tests with cargo-nextest
            anidb-nextest = craneLib.cargoNextest (
              commonArgs
              // {
                inherit cargoArtifacts;
                partitions = 1;
                partitionType = "count";
                cargoNextestPartitionsExtraArgs = "--no-tests=pass";
              }
            );
          };

          packages = {
            inherit anidb-api;
            default = anidb-api;
          };

          devenv.shells.default =
            let
              libs = with pkgs; [
                openssl
              ];
            in
            {
              devenv.root =
                let
                  devenvRootFileContent = builtins.readFile inputs.devenv-root.outPath;
                in
                lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;

              name = "animarr-devenv";
              env = {
                NEXTEST_FAILURE_OUTPUT = "immediate";
                NEXTEST_SUCCESS_OUTPUT = "never";
              };
              packages =
                with pkgs;
                libs
                ++ [
                  cargo-audit
                  cargo-machete
                  cargo-nextest
                  nixfmt-rfc-style
                  taplo
                ];
              languages.rust = {
                enable = true;
                channel = "stable";
                components = [
                  "cargo"
                  "clippy"
                  "rustc"
                  "rustfmt"
                ];
                toolchain = rustToolchain;
                mold.enable = true;
              };
              pre-commit.hooks = {
                clippy = {
                  enable = true;
                  settings.denyWarnings = true;
                  packageOverrides.cargo = rustToolchain;
                  packageOverrides.clippy = rustToolchain;
                };
                deadnix.enable = true;
                nixfmt-rfc-style.enable = true;
                ripsecrets.enable = true;
                rustfmt = {
                  enable = true;
                  packageOverrides.cargo = rustToolchain;
                  packageOverrides.rustfmt = rustToolchain;
                };
                taplo.enable = true;
              };
              containers = lib.mkForce { };
            };
        };
    };
}
