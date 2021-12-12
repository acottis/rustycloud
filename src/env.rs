use std::env;
use crate::error::{Result, Error};


pub fn get() -> Result<String> {
    let args: Vec<_> = env::args().enumerate()
    .filter(|(index, _)| index != &0)
    .map(|(_, arg)| arg)
    .collect();

    let mut vars = String::new();

    for var in env::vars(){
       vars.push_str(&format!("{} --- {}\n", var.0, var.1));
    }

    Ok(format!(
        "ARCH: {ARCH}<br>\
        FAMILY: {FAMILY}<br>\
        OS: {OS}<br>\
        EXE: {EXE}<br>\
        PWD: {PWD}<br>\
        ARGS: {ARGS:?}<br>\
        ### ENVIROMENT VARIABLES ###<br>\
        {VARS}",
        ARCH = env::consts::ARCH,
        FAMILY = env::consts::FAMILY,
        OS = env::consts::OS,
        EXE = env::current_exe().map_err(Error::IO)?.to_str().ok_or(Error::StrConversation)?,
        PWD = env::current_dir().map_err(Error::IO)?.to_str().ok_or(Error::StrConversation)?,
        ARGS = args,
        VARS = vars,
    ))
}

pub fn get_port() -> i16{
    match env::var("PORT"){
        Ok(port) => {
            let port = port.parse();
            match port{
                Ok(port) => port,
                Err(e)=> {
                    println!("Bad PORT variable found in env: {}", e);
                    8080
                }
            }
        }
        _=> {
                println!("No PORT variable found in env");
                8080
        },
    }
}