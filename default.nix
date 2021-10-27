# Example: Custom Holochain And Binaries
#
# The following `shell.nix` file can be used in your project's root folder and activated with `nix-shell`.
# It uses a custom revision and a custom set of binaries to be installed.

{
  holonixPath ?  builtins.fetchTarball { url = "https://github.com/holochain/holonix/archive/e94a377b1f84ecc90a8705031a012793a7fdc93b.tar.gz"; }
}:

let
  holonix = import (holonixPath) {
    include = {
        # making this explicit even though it's the default
        holochainBinaries = true;
    };

    holochainVersionId = "custom";

    holochainVersion = {
      # 0.0.112
      rev = "holochain-v0.0.112";
      sha256 = "03cpn96fn5lhr6shd1w6bd0v4g7pmjppbcwa1g22df299liy681n";
      cargoSha256 = "0miqj8bslfznb858idv47k9rklraizrf56n1n5w9mdzlwnzgvv1f";
      bins = {
        holochain = "holochain";
        hc = "hc";
        kitsune-p2p-proxy = "kitsune_p2p/proxy";
      };

      lairKeystoreHashes = {
        sha256 = "1zq8mpxcy8p7kbj4xl4qhp2hb0fjxakixhzcb4y1rnygc90q9v01";
        cargoSha256 = "1ln0vx1blzjr4p9rqfhcl4b34blk6jiyziz2w5gh09wv2xbhyaa5";
      };
    };
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  buildInputs = with nixpkgs; [
    binaryen
    nodejs-16_x
  ];
}
