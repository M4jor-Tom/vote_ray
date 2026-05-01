{
  pkgs ? import <nixpkgs> {},
  rustPlatform ? pkgs.rustPlatform,
  ...
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rust-analyzer
    pkg-config
    openssl
    postgresql
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rust-src}/lib/rustlib/src/rust/library";
  shellHook = ''
    export RUST_SRC_PATH
    echo "🦀 Rust development environment ready!"
    echo "Run 'cargo build' to build the project"
    echo "Run 'cargo test' to run tests"
  '';
}