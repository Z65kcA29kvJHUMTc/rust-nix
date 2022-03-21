{ pkgs ? import <nixpkgs> { } }:
with pkgs;
pkgs.mkShell {
  nativeBuildInputs = [
    cargo
    cargo-edit
    # libiconv
    openssl
    pkg-config
    rust-analyzer
    rustc
    rustfmt
    # darwin.apple_sdk.frameworks.Security
  ];
}