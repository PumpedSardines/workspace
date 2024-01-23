{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    nixpkgsUnstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flakeUtils.url = "github:numtide/flake-utils";
  };
  outputs =
    { self
    , nixpkgs
    , nixpkgsUnstable
    , flakeUtils
    ,
    }:
    flakeUtils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        pkgsUnstable = nixpkgsUnstable.legacyPackages.${system};
        pkgsFor = nixpkgs.legacyPackages;
      in
      {
        packages.default = pkgsFor.${system}.callPackage ./default.nix { };
        devShells.default = pkgsFor.${system}.callPackage ./shell.nix { };
      }
    );
}
