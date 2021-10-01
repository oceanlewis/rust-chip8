let
  sources = import ./nix/sources.nix;
  rustChannel = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
  rust = rustChannel.rust.override {
    targets = [ ];
    extensions = [
      "clippy-preview"
      "rust-src"
      "rustfmt-preview"
      "rust-analysis"
    ];
  };
in
pkgs.mkShell {
  buildInputs = [
    rust
    pkgs.rust-analyzer
    pkgs.cargo-watch
    pkgs.clippy
  ];
}
