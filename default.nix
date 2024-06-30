# https://nix.dev/tutorials/packaging-existing-software#building-with-nix-build
# default.nix
{ dockerName ? "ghcr.io/dehnert/formationbot-rs/discord-bot", dockerTag ? "test" } :
let
  pkgs = import <nixpkgs> { config = {}; overlays = []; };
  formationbot = pkgs.callPackage ./formationbot.nix { };
in
{
  formationbot = formationbot;
  # https://ryantm.github.io/nixpkgs/builders/images/dockertools/
  docker = pkgs.dockerTools.buildLayeredImage {
    name = "${dockerName}";
    tag = "${dockerTag}";
    config = {
      # See https://github.com/opencontainers/image-spec/blob/main/config.md
      # for semantics
      Cmd = ["${formationbot}/bin/discord-bot"];
      WorkingDir = "/config/";
      Volumes = { "/config/" = { }; };
    };
  };
}

# Running `nix-build default.nix` will run the build and spit out a path
# Running `nix-build default.nix -A docker` will make `result` point to the
# Docker image. Similarly for `-A formationbot`. Without `-A`, we get
# `result` and `result-2`.

# To run:
# nix-build -A docker && docker load < result
# docker run --volume=./container-config/:/config/  ghcr.io/dehnert/formationbot-rs/discord-bot:test
