let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  rustChannel = import ./nix/rust.nix { inherit sources; };

  rust = rustChannel.rust.override {
    targets = [ ];
    extensions = [
      "clippy-preview"
      "rust-src"
      "rustfmt-preview"
      "rust-analysis"
    ];
  };

  inherit (pkgs) lib stdenv;

in
pkgs.mkShell {
  buildInputs = [
    rust
    pkgs.rust-analyzer
    pkgs.cargo-watch
    pkgs.clippy
  ] ++ lib.optionals stdenv.isDarwin [ pkgs.libiconv ];
}
