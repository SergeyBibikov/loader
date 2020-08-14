extern crate serde_json;
extern crate serde;
mod posts;
mod gets;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs,str};
use std::string::String;
use posts::*;
use gets::*;

#[derive(Serialize, Deserialize)]
struct Request {
    protocol: String,
    path: String,
    domain: String,
    port: String,
    method: String,
    max_reqs_per_conn:usize,
    headers: String,    
    path_to_body: String,
    thread_num: usize,
}


fn main() {
    init().unwrap();
}

fn init() -> Result<Request>{
    let mut file = std::env::args();
    file.next();
    //Request data serialization
    let temp = fs::read(file.next().unwrap()).unwrap();
    let data_to_serialize: &str = str::from_utf8(&temp).unwrap();
    let req: Request = serde_json::from_str(data_to_serialize)?;
    println!("Threads: {}\nMethod: {}\nMax_rpc: {}\nLoadSubject: {}://{}:{}/{}\nBody_path: {}",
             req.thread_num,
             req.method,
             req.max_reqs_per_conn,
             req.protocol,
             req.domain,
             req.port,
             req.path,           
             req.path_to_body.trim());

    Ok(req)
}
