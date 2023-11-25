use crate::error::{Error, Result};
use std::env;

pub fn get() -> Result<String> {
    let args: Vec<_> = env::args()
        .enumerate()
        .filter(|(index, _)| index != &0)
        .map(|(_, arg)| arg)
        .collect();

    let mut vars = String::new();

    for var in env::vars() {
        vars.push_str(&format!("{} -> {}<br>\n", var.0, var.1));
    }

    Ok(format!(
        "Hello from Rust: {VERSION}<br>\n\
        ARCH: {ARCH}<br>\n\
        FAMILY: {FAMILY}<br>\n\
        OS: {OS}<br>\n\
        EXE: {EXE}<br>\n\
        PWD: {PWD}<br>\n\
        ARGS: {ARGS:?}<br>\n\
        <br>### ENVIROMENT VARIABLES ###<br>\n\
        {VARS}",
        VERSION = env::var("RUST_VERSION").map_err(Error::EnvVarEmtpy)?,
        ARCH = env::consts::ARCH,
        FAMILY = env::consts::FAMILY,
        OS = env::consts::OS,
        EXE = env::current_exe()
            .map_err(Error::IO)?
            .to_str()
            .ok_or(Error::StrConversation)?,
        PWD = env::current_dir()
            .map_err(Error::IO)?
            .to_str()
            .ok_or(Error::StrConversation)?,
        ARGS = args,
        VARS = vars,
    ))
}

pub fn get_port() -> i16 {
    match env::var("PORT") {
        Ok(port) => {
            let port = port.parse();
            match port {
                Ok(port) => port,
                Err(e) => {
                    println!("ERROR: Bad PORT variable found in env: {}", e);
                    8080
                }
            }
        }
        _ => {
            println!("INFO: No PORT variable found in env");
            8080
        }
    }
}

