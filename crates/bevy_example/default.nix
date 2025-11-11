{ lib, ... }:
{
  perSystem =
    {
      pkgs,
      commonArgs,
      cargoArtifacts,
      ...
    }:
    let
      cargoArtifactsWasm = pkgs.crane.buildDepsOnly (
        commonArgs
        // {
          pname = "deps-wasm";
          doCheck = false;
        }
      );
    in
    {
      packages = {
        bevy-example = pkgs.crane.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "bevy_example";
            cargoExtraArgs = "-p bevy_example";
          }
        );

        bevy-example-web = pkgs.crane.buildTrunkPackage (
          commonArgs
          // {
            inherit cargoArtifactsWasm;
            pname = "bevy_example";
            cargoExtraArgs = "-p bevy_example --target wasm32-unknown-unknown";
            trunkExtraBuildArgs = "--public-url gltf_kun";

            src = lib.cleanSourceWith {
              inherit (commonArgs) src;
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
