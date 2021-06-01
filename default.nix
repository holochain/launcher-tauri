let 
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/3e94163765975f35f7d8ec509b33c3da52661bd1.tar.gz";
    sha256 = "07sl281r29ygh54dxys1qpjvlvmnh7iv1ppf79fbki96dj9ip7d2";
  };
  holonix = import (holonixPath) {
    includeHolochainBinaries = true;
    holochainVersionId = "custom";
    
    holochainVersion = { 
     rev = "363af6d8af8d18e4616f6aa56ad4d1f0fabaafb7";
     sha256 = "0ssjhang6zljs0zrph998zj7582rf0vdb45p855awa7fmzpd4kfa";
     cargoSha256 = "15g0ndb64hcfqyqn5pb01vnipg4gxlm0p543z2vr407dgrwfwvhw";
     bins = {
       holochain = "holochain";
       hc = "hc";
     };
    };
    holochainOtherDepsNames = ["lair-keystore"];
  };
in holonix.main
