{ sources ? import ./nix/sources.nix { }
, pkgs ? import sources.nixpkgs { }
, gitignore ? import sources.gitignore { }
}:
with pkgs;
rustPlatform.buildRustPackage rec {
  name = "swaylayout";

  src = gitignore.gitignoreSource ./.;

  cargoSha256 = "05zl4whc4aglg2l0m96z1r60zd9ay506xcifnirymjydgssqjqrx";
}
