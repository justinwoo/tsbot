{ pkgs ? import <nixpkgs> {} }:

let
  dynamic-linker = pkgs.stdenv.cc.bintools.dynamicLinker;

in
pkgs.stdenv.mkDerivation rec {
  name = "tsbot";

  src = pkgs.fetchurl {
    url = "https://github.com/justinwoo/tsbot/releases/download/2020-01-31/tsbot";
    sha256 = "1k1q1a2jvcvml3gzjbywq2y2381xhhxwwva0c3mkbffxfws4vv7a";
  };

  buildInputs = [ pkgs.makeWrapper ];

  dontStrip = true;

  libPath = pkgs.lib.makeLibraryPath [
    pkgs.glibc
    pkgs.openssl_1_1.out
  ];

  unpackPhase = ''
    mkdir -p $out/bin
    TARGET=$out/bin/tsbot

    cp $src $TARGET
    chmod +x $TARGET

    patchelf $TARGET \
      --interpreter ${dynamic-linker} \
      --set-rpath ${libPath}
  '';

  dontInstall = true;
}
