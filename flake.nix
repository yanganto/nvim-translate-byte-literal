{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11-small";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        cargoToml = "${self}/Cargo.toml";
        manifest = builtins.fromTOML (builtins.readFile cargoToml);
      in
      {
        devShell = pkgs.mkShell ({
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
          ];
        });
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          inherit (manifest.package) name version;
          src = pkgs.lib.cleanSource ./.;
          cargoHash = "sha256-NAwv7Nt95oP0V7bqVmi8kish0CXutH7BgVWlpeGNIzY=";
        };
      }
    );
}
