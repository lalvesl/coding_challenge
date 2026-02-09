{
  pkgs ? import <nixpkgs> { },
}:

let
  flake = builtins.getFlake (toString ./.);
  system = pkgs.stdenv.hostPlatform.system;
in
pkgs.mkShell {
  inputsFrom = [
    flake.devShells.${system}.default
  ];

  shellHook = "";
}
