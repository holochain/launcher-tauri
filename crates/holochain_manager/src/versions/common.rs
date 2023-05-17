use url2::Url2;

pub fn bootstrap_service() -> Url2 {
  url2::url2!("https://bootstrap.holo.host")
}

pub fn signaling_server() -> String {
  String::from("wss://signal.holo.host")
}

