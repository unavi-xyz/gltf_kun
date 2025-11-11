{
  inputs = {
    # Nix
    flake-parts = {
      inputs.nixpkgs-lib.follows = "nixpkgs";
      url = "github:hercules-ci/flake-parts";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    treefmt-nix.url = "github:numtide/treefmt-nix";

    # Rust
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, systems, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      { ... }:
      {
        systems = import systems;

        imports = [
          inputs.treefmt-nix.flakeModule
          ./crates/bevy_example
        ];

        perSystem =
          {
            config,
            lib,
            pkgs,
            system,
            ...
          }:
          let
            commonArgs = {
              src = lib.cleanSource ./.;
              strictDeps = true;

              buildInputs =
                lib.optionals pkgs.stdenv.isLinux (
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
                ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs; [ darwin.apple_sdk.frameworks.Cocoa ]);

              nativeBuildInputs =
                with pkgs;
                [
                  binaryen
                  clang
                  mold
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

            cargoArtifacts = pkgs.crane.buildDepsOnly (commonArgs // { pname = "deps"; });
          in
          {
            _module.args.pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.fenix.overlays.default

                (
                  self: _:
                  let
                    toolchain = (
                      with self.fenix;
                      combine [
                        stable.toolchain
                        targets.wasm32-unknown-unknown.stable.rust-std
                      ]
                    );
                  in
                  {
                    crane = (inputs.crane.mkLib self).overrideToolchain toolchain;
                  }
                )

              ];
            };

            _module.args = { inherit commonArgs cargoArtifacts; };

            checks = {
              clippy = pkgs.crane.cargoClippy (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  pname = "clippy";
                }
              );

              doc = pkgs.crane.cargoDoc (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  pname = "doc";
                }
              );
            };

            packages.default = config.packages.bevy-example-web;

            treefmt.programs = {
              actionlint.enable = true;
              deadnix.enable = true;
              mdformat.enable = true;
              nixfmt = {
                enable = true;
                strict = true;
              };
              rustfmt.enable = true;
              statix.enable = true;
              taplo.enable = true;
              yamlfmt.enable = true;
            };

            devShells.default = pkgs.crane.devShell {
              packages =
                config.packages
                |> lib.attrValues
                |> lib.flip pkgs.lib.forEach (x: x.buildInputs ++ x.nativeBuildInputs)
                |> lib.concatLists
                |> lib.unique
                |> (
                  basePkgs:
                  basePkgs
                  ++ (with pkgs; [
                    cargo-edit
                    cargo-machete
                    cargo-nextest
                    cargo-rdme
                    cargo-release
                    cargo-watch
                    cargo-workspaces
                    rust-analyzer
                  ])
                );

              LD_LIBRARY_PATH =
                config.packages
                |> lib.attrValues
                |> lib.filter (x: x ? buildInputs)
                |> lib.flip pkgs.lib.forEach (x: x.buildInputs)
                |> lib.concatLists
                |> lib.unique
                |> lib.makeLibraryPath;
            };
          };
      }
    );
}
