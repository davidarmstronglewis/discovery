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
  buildInputs = [ rust ]
    ++
    (with pkgs; [
      cargo-watch
      clippy
      gcc-arm-embedded
      minicom
      openocd
      rust-analyzer
    ])
    ++
    lib.optionals stdenv.isDarwin (with pkgs; [
      libiconv
    ]);
}
