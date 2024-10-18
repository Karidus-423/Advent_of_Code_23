{
  description = "A very basic flake";

  inputs = {
    devshell.url = "github:numtide/devshell";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    systems.url = "github:nix-systems/default";
    zig-overlay.url = "github:mitchellh/zig-overlay";
  };

  outputs = inputs @ {
    self,
    devshell,
    flake-parts,
    nixpkgs,
    systems,
    zig-overlay,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        devshell.flakeModule
      ];
      systems = import systems;
      perSystem = {
        config,
        pkgs,
        system,
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.zig-overlay.overlays.default
          ];
        };

        devshells.default = {
          name = "Advent23";
          env = [];
          packages = with pkgs; [
            zigpkgs.master
			zls
          ];
        };
      };
    };
}


