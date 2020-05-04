{ sources ? import ./nix/sources.nix { }
, pkgs ? import sources.nixpkgs { }
, niv ? import sources.niv { }
}:
with pkgs;
mkShell { buildInputs = [ niv.niv ]; }
