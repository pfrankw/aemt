{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # static cross-compilation tools
    pkgsStatic.stdenv.cc

    # ALSA libraries for musl.. yes pkgsStatic often means musl
    pkgsStatic.alsa-lib
    pkgsStatic.alsa-lib.dev
    pkgsStatic.pkg-config
  ];

  shellHook = ''
    export PKG_CONFIG_ALLOW_CROSS=1
    export PKG_CONFIG_PATH="${pkgs.pkgsStatic.alsa-lib.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
    # export CC_x86_64_unknown_linux_musl="${pkgs.pkgsStatic.stdenv.cc}/bin/musl-gcc"
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="${pkgs.pkgsStatic.stdenv.cc}/bin/x86_64-unknown-linux-musl-cc"
  '';
}
