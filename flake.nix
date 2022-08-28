{
  description = "A simple Rust-based stack machine";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk/master";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        n = pkgs.callPackages naersk {};
      in
      {
        defaultPackage = n.buildPackage {
          pname = "ito";
          doCheck = true;
          src = ./.;
        };

        defaultApp = flake-utils.lib.mkApp {
          drv = self.defaultPackage.${system};
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            cargo-insta
            rust-analyzer
            rustPackages.clippy
            rustc
            rustfmt
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      }
    );
}
