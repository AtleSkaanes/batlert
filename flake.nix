{
  description = "A GTK based battery popup";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      formatter.x86_64-linux = pkgs.nixfmt-rfc-style;
      packages.x86_64-linux.default = pkgs.callPackage ./nix/package.nix { };
    };
}
