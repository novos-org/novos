{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.xz
    pkgs.gnumake
    pkgs.b3sum
    pkgs.coreutils 
  ];

  shellHook = ''
    export TARGET_OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    export TARGET_ARCH=$(uname -m)
    export PATH="$PWD:$PATH"
    echo "novos shell.nix"
    echo "$TARGET_OS - $TARGET_ARCH"
    echo "run: build.sh build, build.sh dist, or cargo commands."
  '';
}
