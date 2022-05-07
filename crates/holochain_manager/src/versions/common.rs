use url2::Url2;

pub fn proxy_url() -> Url2 {
  url2::url2!("kitsune-proxy://f3gH2VMkJ4qvZJOXx0ccL_Zo5n-s_CnBjSzAsEHHDCA/kitsune-quic/h/137.184.142.208/p/5788/--")
}

pub fn boostrap_service() -> Url2 {
  url2::url2!("https://bootstrap.holo.host")
}
