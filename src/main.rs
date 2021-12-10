use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;

fn main() {
    println!("Hello, world!");
    
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Cannot bind TCP listener");

    for stream in listener.incoming(){
        match stream{
            Ok(s) => {
                thread::spawn(move ||
                    http_main(s)
                );
            },
            _ => {} 
        }
    }
}

fn http_main(mut stream: TcpStream) -> Result<(), std::io::Error>{
    let mut msg = String::new();

    let mut reader = BufReader::new(&stream);
    reader.read_line(&mut msg)?;
    println!("New Request, {:?}, data: {:?}", stream.peer_addr(), msg);
    stream.write(b"HTTP/1.1 200\r\nContent-Length:13\r\n\r\nHello World\r\n")?;

    Ok(())
}
