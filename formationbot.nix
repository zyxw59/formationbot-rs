{ stdenv, lib, rustPlatform, pkg-config, pango, gdk-pixbuf, glib, cairo, libxml2 }:

let
  pkgs = import <nixpkgs> {};
  fs = lib.fileset;
in rustPlatform.buildRustPackage rec {
  pname = "formationbot-rs";
  version = "1.0";

  src = fs.toSource {
    #fileset = fs.intersection ./discord-bot (fs.gitTracked ./.);
    root = ./.;
    fileset = (fs.gitTracked ./.);
  };
  buildAndTestSubdir = "discord-bot";

  cargoLock = {
    lockFile = ./discord-bot/Cargo.lock;
  };

  meta = with lib; {
    description = "FormationBot renders square dance formations as pictures, to aid discussions of calling";
    homepage = "https://github.com/zyxw59/formationbot-rs";
    platforms = platforms.linux;
  };

  buildInputs = [
    pango
    gdk-pixbuf
    glib
    cairo
    libxml2
  ];
  nativeBuildInputs = [
    pkg-config
  ];
}
