let
  holonixRev = "bcb7cbedfc06026181552a7d64db731c0398165c";

  holonixPath = builtins.fetchTarball "https://github.com/holochain/holonix/archive/${holonixRev}.tar.gz";
  holonix = import (holonixPath) {};
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    # Additional packages go here
    nodejs-16_x
    caddy
    glib
    cairo
    gtksourceview3
    gtk3
    libappindicator
    pango
    squashfsTools
    atk
    gdk-pixbuf
    libsoup
    pkgconfig
    llvmPackages.libclang
    llvmPackages.libcxxClang
    clang
    zlib
  ];
}