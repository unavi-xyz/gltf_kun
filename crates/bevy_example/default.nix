{ lib, ... }:
{
  perSystem =
    { pkgs, ... }:
    let
      pname = "bevy_example";

      src = lib.fileset.toSource rec {
        root = ../..;
        fileset = lib.fileset.unions [
          (pkgs.crane.fileset.commonCargoSources root)
          ../../index.html
          ../../assets
        ];
      };

      baseCargoArgs = {
        inherit pname src;
        strictDeps = true;
      };

      nativeCargoArgs = baseCargoArgs // rec {
        cargoExtraArgs = "-p ${pname}";

        linkedInputs = pkgs.lib.optionals pkgs.stdenv.isLinux (
          with pkgs;
          [
            alsa-lib
            libxkbcommon
            udev
            vulkan-loader
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ]
        );

        buildInputs =
          pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [ darwin.apple_sdk.frameworks.Cocoa ])
          ++ linkedInputs;

        nativeBuildInputs =
          with pkgs;
          [
            clang
            llvmPackages.bintools
            mold
            pkg-config
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
            alsa-lib
            alsa-lib.dev
          ];
      };

      wasmCargoArgs = baseCargoArgs // {
        cargoExtraArgs = "-p ${pname} --target wasm32-unknown-unknown";
        doCheck = false;

        nativeBuildInputs = with pkgs; [
          binaryen
          clang
          llvmPackages.bintools
          mold
          trunk
          wasm-bindgen-cli
          wasm-tools
        ];

        CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
      };

      cargoArtifactsNative = pkgs.crane.buildDepsOnly nativeCargoArgs;
      cargoArtifactsWasm = pkgs.crane.buildDepsOnly wasmCargoArgs;
    in
    {
      checks = {
        "${pname}-clippy" = pkgs.crane.cargoClippy (
          nativeCargoArgs // { cargoArtifacts = cargoArtifactsNative; }
        );

        "${pname}-doc" = pkgs.crane.cargoDoc (
          nativeCargoArgs // { cargoArtifacts = cargoArtifactsNative; }
        );
      };

      packages = {
        bevy-example = pkgs.crane.buildPackage (
          nativeCargoArgs // { cargoArtifacts = cargoArtifactsNative; }
        );

        bevy-example-web = pkgs.crane.buildTrunkPackage (
          wasmCargoArgs
          // {
            cargoArtifacts = cargoArtifactsWasm;
            trunkIndexPath = "index.html";
            trunkExtraBuildArgs = "--public-url gltf_kun";

            src = lib.cleanSourceWith {
              inherit src;
              filter =
                path: type:
                (lib.hasSuffix ".html" path)
                || (lib.hasInfix "/assets/" path)
                || (pkgs.crane.filterCargoSources path type);
            };

            inherit (pkgs) wasm-bindgen-cli;
          }
        );
      };
    };
}
