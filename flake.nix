{
  description = "thumbfetch - youtube thumbnail downloader in rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "thumbfetch";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];

          RUSTFLAGS = "-C target-cpu=native";
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.pkg-config
            pkgs.openssl
            pkgs.cargo
          ];
        };
      }
    );
}
