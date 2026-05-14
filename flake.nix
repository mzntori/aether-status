{
  description = "custom status bar";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in
    {
      packages."x86_64-linux".default = pkgs.rustPlatform.buildRustPackage rec {
        pname = "aether-status";
        version = "0.1.0";
        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;
      };

      devShells."x86_64-linux".default = pkgs.mkShell {
        name = "ae-st-dev";
        packages = with pkgs; [
          rustup
          rustc
          cargo
        ];
      };
    };
}
