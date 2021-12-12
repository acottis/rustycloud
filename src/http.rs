use std::net::TcpStream;
use crate::error::{Result, Error};
use std::io::{Write, BufReader, BufRead};

pub struct Http{
    stream: TcpStream,
}

impl Http{
    pub fn new(stream: TcpStream) -> Result<Self>{
        Ok(Self{
            stream,
        })
    }

    pub fn read(&self) -> Result<String>{
        let mut msg = String::new();

        let mut reader = BufReader::new(&self.stream);
        reader.read_line(&mut msg).map_err(Error::IO)?;

        println!("New Request, {:?}, data: {:?}", &self.stream.peer_addr(), msg);
        Ok(msg)
    }

    pub fn write(&mut self, body: &str) -> Result<()> {

        let content_len = body.len();

        let headers = format!("HTTP/1.1 200\r\nContent-Length:{}\r\n\r\n", content_len);

        let res = format!(
            "{headers}\
            {body}",
            headers = headers,
            body = body,
        );

        self.stream.write(res.as_bytes()).map_err(Error::IO)?;
        Ok(())
    }
}