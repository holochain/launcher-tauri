let 
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/3e94163765975f35f7d8ec509b33c3da52661bd1.tar.gz";
    sha256 = "07sl281r29ygh54dxys1qpjvlvmnh7iv1ppf79fbki96dj9ip7d2";
  };
  holonix = import (holonixPath) {
    includeHolochainBinaries = true;
    holochainVersionId = "custom";
    
    holochainVersion = { 
     rev = "278a9dc2d29b4f31188fdf8c5543529cee6760a5";
     sha256 = "sha256:08sxvsawwdjbz3xj6h8dvdab5lpkhbwhcxr4rz47i7kqxv5lmpx9";
     cargoSha256 = "sha256:07gdvccvjbg5zina751r8d8ga87pb84ss2a5ib453ykwparr53i3";
     bins = {
       holochain = "holochain";
       hc = "hc";
     };
    };
    holochainOtherDepsNames = ["lair-keystore"];
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  buildInputs = with nixpkgs; [
    caddy
    glib
    cairo
    pango
    atk
    gdk-pixbuf
    libsoup
    gtk3
    pkgconfig
    webkitgtk
    gtksourceview3
    llvmPackages.libclang
    llvmPackages.libcxxClang
    clang
    zlib
    libappindicator
    squashfsTools
    nix-index
  ];


  shellHook = ''
    export LIBCLANG_PATH="${nixpkgs.llvmPackages.libclang}/lib";
    unset SOURCE_DATE_EPOCH;

    rm -rf src-tauri/bins/
    mkdir src-tauri/bins

    HOLOCHAIN_PATH=$(which holochain)
    cp $HOLOCHAIN_PATH src-tauri/bins/holochain-x86_64-unknown-linux-gnu

    LAIR_PATH=$(which lair-keystore)
    cp $LAIR_PATH src-tauri/bins/lair-keystore-x86_64-unknown-linux-gnu

    CADDY_PATH=$(which caddy)
    cp $CADDY_PATH src-tauri/bins/caddy-x86_64-unknown-linux-gnu

  '';
}