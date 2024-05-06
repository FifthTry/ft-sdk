{
  description = "ft-sdk";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      toolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rustfmt" "clippy" "rust-src" ];
        targets = [ "wasm32-unknown-unknown" ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "sdk-shell";
        nativeBuildInputs = with pkgs; [
          pkg-config
          openssl.dev
        ];
        buildInputs = with pkgs; [
          toolchain
          rust-analyzer-unwrapped
          cargo-expand
        ];

        RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
      };

      formatter.${system} = pkgs.nixpkgs-fmt;
    };
}