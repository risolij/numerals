{
  description = "billing";
  nixConfig.bash-prompt = "\[nix-develop\]$ ";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        lib = nixpkgs.lib;
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = [
            pkg-config
            gcc
            openssl
            rust-analyzer
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = ["rust-src"];
            }))
          ];
        };
      }
    );
}
