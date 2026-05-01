{
  description = "Vote Ray Backend - Rust voting system with SeaORM";

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
          name = "vote-ray-backend";

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
            echo "🦀 Vote Ray Backend - Nix development environment"
            echo "📊 PostgreSQL: podman run -d -p 5432:5432 -e POSTGRES_DB=vote_ray -e POSTGRES_USER=vote_ray -e POSTGRES_PASSWORD=dev postgres:15"
            echo "🚀 Start development: cargo run"
            echo "📚 Swagger docs: http://localhost:3000/swagger-ui"

            # Database defaults
            export DATABASE_URL="postgresql://vote_ray:dev@localhost:5432/vote_ray"
          '';
        };
      }
    );
}
