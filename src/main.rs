use std::net::{TcpListener};

use std::thread;

mod env;

mod http;
use http::Http;

mod error;

static LISTENING_ADDR: &'static str = "0.0.0.0";

fn main() {    
    let port = env::get_port();

    let bind_addr = format!("{}:{}", &LISTENING_ADDR, &port);

    let listener = TcpListener::bind(&bind_addr).expect("Cannot bind TCP listener");
    println!("Listening on -> {}", &bind_addr);    

    for stream in listener.incoming(){
        match stream{
            Ok(s) => {
                thread::spawn(||
                    http_main(Http::new(s).expect("Issue with stream"))
                );
            },
            _ => {} 
        }
    }
}

fn http_main(mut http: Http) -> Result<(), std::io::Error>{

    let env = env::get().expect("Could not get env variables");

    http.read().unwrap();
    
    http.write(&env).unwrap();

    Ok(())
}
