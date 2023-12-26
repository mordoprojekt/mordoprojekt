# run nix develop --impure
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv/v0.6.3";
  };

  outputs = { self, nixpkgs, devenv, systems, ... } @ inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
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

                # https://search.nixos.org/packages?channel=unstable
                packages = with pkgs; [
                  rustc
                  cargo
                ];
              }
            ];
          };
        });
    };
}
