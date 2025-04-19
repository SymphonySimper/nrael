{
  description = "nrael dev shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    { ... }@inputs:
    let
      forAllSystems =
        f:
        inputs.nixpkgs.lib.genAttrs (import inputs.systems) (
          system:
          f {
            pkgs = import inputs.nixpkgs { inherit system; };
          }
        );

      mkShell =
        pkgs: config:
        let
          lib = pkgs.lib;
        in
        pkgs.mkShell (
          {
            buildInputs = [
              pkgs.bashInteractive # do not remove
            ];

            packages = builtins.concatLists [
              [
                pkgs.just
              ]

              (lib.lists.optionals (builtins.hasAttr "packages" config) config.packages)
            ];

            shellHook = lib.strings.optionalString (builtins.hasAttr "shellHook" config) config.shellHook;
          }
          // (lib.attrsets.optionalAttrs (builtins.hasAttr "env" config) config.env)
          // (lib.attrsets.optionalAttrs (builtins.hasAttr "ld" config) {
            LD_LIBRARY_PATH = lib.makeLibraryPath config.ld;
          })
        );
    in
    {
      devShells = forAllSystems (
        { pkgs }:
        {
          default = mkShell pkgs { };

          rust = mkShell pkgs {
            env.RUST_BACKTRACE = "1";

            packages = with pkgs; [
              rustup
              sccache
            ];
          };
        }
      );
    };
}
