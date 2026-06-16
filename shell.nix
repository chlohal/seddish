let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [
      (import "${fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz"}/overlay.nix")
      (import rust-overlay)
      (_: prev: {
        my-rust-toolchain = prev.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      })
    ];
  };
in
pkgs.callPackage (
  {
    mkShell,
    my-rust-toolchain,
  }:
  mkShell {
    strictDeps = true;
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    nativeBuildInputs = [
      pkgs.openssl
      pkgs.perf
      pkgs.rust-analyzer-nightly
      pkgs.pkg-config
      my-rust-toolchain
    ];
  }
) { }
