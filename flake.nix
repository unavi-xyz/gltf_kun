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
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (localSystem:
      let
        pkgs = import nixpkgs {
          inherit localSystem;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchain =
          pkgs.pkgsBuildHost.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (lib.hasSuffix ".proto" path) || (lib.hasSuffix ".wit" path)
              || (craneLib.filterCargoSources path type);
          };

          strictDeps = true;

          buildInputs = lib.optionals pkgs.stdenv.isLinux (with pkgs; [
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
          ]) ++ lib.optionals pkgs.stdenv.isDarwin
            (with pkgs; [ pkgs.darwin.apple_sdk.frameworks.Cocoa ]);

          nativeBuildInputs = with pkgs;
            [
              binaryen
              cargo-auditable
              cargo-component
              clang
              cmake
              nodePackages.prettier
              pkg-config
              protobuf
              trunk
              wasm-bindgen-cli
              wasm-tools
            ] ++ lib.optionals (!pkgs.stdenv.isDarwin)
            (with pkgs; [ alsa-lib alsa-lib.dev ]);

        };

        commonShell = {
          checks = self.checks.${localSystem};
          packages = with pkgs; [ cargo-watch rust-analyzer ];
        };

        cargoArtifacts =
          craneLib.buildDepsOnly (commonArgs // { pname = "deps"; });

        cargoClippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          pname = "clippy";
        });

        cargoDoc = craneLib.cargoDoc (commonArgs // {
          inherit cargoArtifacts;
          pname = "doc";
        });

        bevy_gltf_kun = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "bevy_gltf_kun";
        });

        gltf_kun = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "gltf_kun";
        });
      in {
        checks = { inherit gltf_kun bevy_gltf_kun cargoClippy cargoDoc; };

        packages = {
          bevy_gltf_kun = bevy_gltf_kun;
          gltf_kun = gltf_kun;

          default = pkgs.symlinkJoin {
            name = "all";
            paths = [ bevy_gltf_kun gltf_kun ];
          };
        };

        devShells.default = craneLib.devShell commonShell;
      });
}
