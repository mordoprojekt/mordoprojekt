# To get developer environment run:
# $ nix develop --impure
{
  description = "Mordoprojekt discord bot!";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    devenv.url = "github:cachix/devenv/v0.6.3";
  };

  outputs = { self, nixpkgs, devenv, ... } @ inputs:
    let
      supportedSystems = [ "aarch64-darwin" "x86_64-linux" ];
      forEachSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
    in
    {
      devShells = forEachSystem (system:
        let
            pkgs = import nixpkgs {};
        in
        {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;

            modules = [
              {
                # nice shell
                starship.enable = true;

                env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
                # https://search.nixos.org/packages?channel=unstable
                packages = with pkgs; [
                  rustc
                  cargo
                  rust-analyzer
                  rustfmt
                  act
                  gcc
                  sqlx-cli
                ];
              }
            ];
          };
        });
    };
}
