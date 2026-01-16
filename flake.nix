{
  description = "Rust REST API backend with Postgres + opencode (Hugging Face models)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          rust-overlay.overlays.default
        ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rustfmt" "clippy" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "rust-rest-backend";

          buildInputs = [
            rustToolchain

            # Database
            pkgs.postgresql
            pkgs.sqlx-cli

            # Tooling
            pkgs.pkg-config
            pkgs.openssl

            # Node + opencode
            pkgs.nodejs_20
          ];

          shellHook = ''
            echo "🦀 Rust REST API dev shell"
            echo "📦 Postgres available via pg_ctl"
            echo "🤖 opencode available via npx opencode"

            # Database defaults
            export DATABASE_URL="postgres://postgres:postgres@localhost:5432/app"

            # Hugging Face (set your token!)
            export HUGGINGFACE_API_TOKEN="__SET_ME__"

            # Example free models
            export OPENCODE_MODEL="mistralai/Mistral-7B-Instruct-v0.2"

            # Make sure local node tools are usable
            export PATH="$PWD/node_modules/.bin:$PATH"
          '';
        };
      }
    );
}
