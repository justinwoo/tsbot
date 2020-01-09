{ pkgs ? import <nixpkgs> {} }:

let
  dynamic-linker = pkgs.stdenv.cc.bintools.dynamicLinker;

in
pkgs.stdenv.mkDerivation rec {
  name = "nixpkgs-fmt";

  src = pkgs.fetchurl {
    url = "https://github.com/justinwoo/tsbot/releases/download/2020-01-09/tsbot";
    sha256 = "0d517ifw9c0g2pp68g8v383v7lmhn47bspas13s3ja1ncqgnqnqc";
  };

  buildInputs = [ pkgs.makeWrapper ];

  dontStrip = true;

  libPath = pkgs.lib.makeLibraryPath [ pkgs.glibc ];

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
