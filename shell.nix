{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    libiconv
  ];

  # Ensure the linker can find libiconv
  LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.libiconv ];
}
