{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    # ALSA libraries
    alsa-lib
    alsa-lib.dev
  ];

  shellHook = ''
    export PKG_CONFIG_ALLOW_CROSS=1
    export PKG_CONFIG_PATH="${pkgs.alsa-lib.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
  '';
}
