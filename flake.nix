# To get developer environment
# $ nix develop --impure
{
  description = "Mordoprojekt discord bot!";

  inputs = {
    # rust-overlay.url = "github:oxalica/rust-overlay";
    # flake-utils.follows = "rust-overlay/flake-utils";
    
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    # nixpkgs.follows = "rust-overlay/nixpkgs";
    
    devenv.url = "github:cachix/devenv/v0.6.3";
    
    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, devenv, naersk, ... } @ inputs:
    let
      cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      supportedSystems = [ "aarch64-darwin" "x86_64-linux" ];
      forEachSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
    in
    {
      overlay = final: prev: {
        "${cargoToml.package.name}" = final.callPackage ./. { inherit naersk; };
      };

      packages = forEachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              self.overlay
            ];
          };
        in
        {
          "${cargoToml.package.name}" = pkgs."${cargoToml.package.name}";
        });

      defaultPackage = forEachSystem (system: (import nixpkgs {
        inherit system;
        overlays = [ self.overlay ];
      })."${cargoToml.package.name}");

            checks = forEachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              self.overlay
            ];
          };
        in
        {
          format = pkgs.runCommand "check-format"
            {
              buildInputs = with pkgs; [ rustfmt cargo ];
            } ''
            ${pkgs.rustfmt}/bin/cargo-fmt fmt --manifest-path ${./.}/Cargo.toml -- --check
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
            touch $out # it worked!
          '';
          "${cargoToml.package.name}" = pkgs."${cargoToml.package.name}";
        });

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
                ];
              }
            ];
          };
        });
    };
}
