use std::net::TcpListener;

use std::thread;

mod env;

mod http;
use http::Http;

mod error;

static LISTENING_ADDR: &'static str = "0.0.0.0";

const INDIE_AUTH: &'static str = r#"<html>
    <link href="https://github.com/acottis" rel="me">
</html>"#;

fn main() {
    let port = env::get_port();

    let bind_addr = format!("{}:{}", &LISTENING_ADDR, &port);

    let listener = TcpListener::bind(&bind_addr).expect("Cannot bind TCP listener");
    println!("INFO: Listening on -> {}", &bind_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(|| http_main(Http::new(s).expect("Issue with stream")));
            }
            _ => {}
        }
    }
}

fn http_main(mut http: Http) -> Result<(), std::io::Error> {
    http.read().unwrap();
    http.write(INDIE_AUTH).unwrap();

    Ok(())
}
