{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    just
    cargo
    rustc
    pkg-config
    openssl
  ];
}
