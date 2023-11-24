{
  description = "Template for Holochain app development";

  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";
    versions.url = "github:holochain/holochain?dir=versions/weekly";

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "versions";
    };
  };

  outputs = inputs @ {...}:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
    {
      inherit inputs;
    }
    {
      systems = builtins.attrNames inputs.holochain.devShells;
      perSystem = {
        self',
        inputs',
        config,
        pkgs,
        system,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          inputsFrom = [inputs'.holochain.devShells.rustDev];
          packages = with pkgs; [
            nodejs-18_x
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
            pkg-config
            llvmPackages.libclang
            llvmPackages.libcxxClang
            clang
            zlib
          ];
        };
      };
    };
}