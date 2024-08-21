{ pkgs ? (import <nixpkgs> {
    config.allowUnfree = true;
}) }:

pkgs.stdenv.mkDerivation {
  name = "hyprland-workspaces";

  buildInputs = with pkgs; [
    rustup
    cargo
  ];
}