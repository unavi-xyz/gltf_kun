{
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      localSystem:
      let
        pkgs = import nixpkgs {
          inherit localSystem;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = pkgs.lib.cleanSource ./.;

          strictDeps = true;

          buildInputs =
            pkgs.lib.optionals pkgs.stdenv.isLinux (
              with pkgs;
              [
                alsa-lib
                alsa-lib.dev
                libxkbcommon
                udev
                vulkan-loader
                wayland
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
              ]
            )
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [ darwin.apple_sdk.frameworks.Cocoa ]);

          nativeBuildInputs =
            with pkgs;
            [
              binaryen
              cargo-auditable
              nodePackages.prettier
              pkg-config
              trunk
              wasm-bindgen-cli
              wasm-tools
            ]
            ++ lib.optionals (!stdenv.isDarwin) [
              alsa-lib
              alsa-lib.dev
            ];
        };

        commonShell = {
          checks = self.checks.${localSystem};
          packages = with pkgs; [
            cargo-rdme
            cargo-watch
            rust-analyzer
          ];

          LD_LIBRARY_PATH = (pkgs.lib.makeLibraryPath commonArgs.buildInputs);
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { pname = "deps"; });

        cargoArtifactsWasm = craneLib.buildDepsOnly (
          commonArgs
          // {
            pname = "deps-wasm";
            doCheck = false;
          }
        );

        cargoClippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "clippy";
          }
        );

        cargoDoc = craneLib.cargoDoc (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "doc";
          }
        );

        bevy_gltf_kun = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "bevy_gltf_kun";
            cargoExtraArgs = "-p bevy_gltf_kun";
          }
        );

        gltf_kun = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "gltf_kun";
            cargoExtraArgs = "-p gltf_kun --all-features";
          }
        );

        web = craneLib.buildTrunkPackage (
          commonArgs
          // {
            inherit cargoArtifactsWasm;
            pname = "bevy_example";
            cargoExtraArgs = "-p bevy_example --target wasm32-unknown-unknown";
            trunkExtraBuildArgs = "--public-url gltf_kun";

            src = pkgs.lib.cleanSourceWith {
              src = ./.;
              filter =
                path: type:
                (pkgs.lib.hasSuffix ".html" path)
                || (pkgs.lib.hasInfix "/assets/" path)
                || (craneLib.filterCargoSources path type);
            };

            wasm-bindgen-cli = pkgs.wasm-bindgen-cli;
          }
        );
      in
      {
        checks = {
          inherit
            gltf_kun
            bevy_gltf_kun
            web
            cargoClippy
            cargoDoc
            ;
        };

        apps = {
          generate-readme = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "generate-readme" ''
              cd crates

              for folder in */; do
                (cd $folder && cargo rdme)
              done
            '';
          };
        };

        packages = {
          bevy_gltf_kun = bevy_gltf_kun;
          gltf_kun = gltf_kun;
          web = web;

          default = pkgs.symlinkJoin {
            name = "all";
            paths = [
              bevy_gltf_kun
              gltf_kun
              web
            ];
          };
        };

        devShells.default = craneLib.devShell commonShell;

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
