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
    let req_d = init().unwrap();    
    let request_threads:Vec<std::thread::JoinHandle<()>>;
    if req_d.protocol=="http"&&req_d.method=="GET"{
        request_threads = gen_http_get_reqs(&req_d);
    }
    else if req_d.protocol=="https"&&req_d.method=="GET"{
        request_threads = gen_https_get_reqs(&req_d);
    }
    else if req_d.protocol=="http"&&req_d.method=="POST"{
        request_threads = gen_http_post_reqs(&req_d);
    }
    else {
        request_threads = gen_https_post_reqs(&req_d);
    }
    for j in request_threads{ j.join().unwrap(); } 
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
fn gen_http_get_reqs(input: &Request) -> Vec<std::thread::JoinHandle<()>>{
    let mut request_threads = vec![];
    let reqs_p_conn = input.max_reqs_per_conn;
    for _ in 0..input.thread_num{
        let path = input.path.clone();
        let domain = input.domain.clone();
        let port = input.port.clone();
        let headers = input.headers.clone();
        //let body = req_body.clone();
        request_threads.push(std::thread::spawn(move ||{
            get_req(&path,&domain,&port,&headers,&reqs_p_conn);
            //tls_post_req(&path,&domain,&port,&body,&headers);                
        }));
    }
    request_threads
}

fn gen_https_get_reqs(input: &Request) -> Vec<std::thread::JoinHandle<()>>{
    let mut request_threads = vec![];
    let reqs_p_conn = input.max_reqs_per_conn;
    for _ in 0..input.thread_num{
        let path = input.path.clone();
        let domain = input.domain.clone();
        let port = input.port.clone();
        let headers = input.headers.clone();
        request_threads.push(std::thread::spawn(move ||{
            tls_get_req(&path,&domain,&port,&headers,&reqs_p_conn);                
        }));
    }
    request_threads
}

fn gen_http_post_reqs(input: &Request) -> Vec<std::thread::JoinHandle<()>>{
    let mut request_threads = vec![];
    let reqs_p_conn = input.max_reqs_per_conn;
    let temp_req_body = fs::read(&input.path_to_body).unwrap();
    let req_body = String::from_utf8(temp_req_body).unwrap();
    for _ in 0..input.thread_num{
        let path = input.path.clone();
        let domain = input.domain.clone();
        let port = input.port.clone();
        let headers = input.headers.clone();
        let body = req_body.clone();
        request_threads.push(std::thread::spawn(move ||{
            post_req(&path,&domain,&port,&body,&headers,&reqs_p_conn);               
        }));
    }
    request_threads
}

fn gen_https_post_reqs(input: &Request) -> Vec<std::thread::JoinHandle<()>>{
    let mut request_threads = vec![];
    let reqs_p_conn = input.max_reqs_per_conn;
    let temp_req_body = fs::read(&input.path_to_body).unwrap();
    let req_body = String::from_utf8(temp_req_body).unwrap();
    for _ in 0..input.thread_num{
        let path = input.path.clone();
        let domain = input.domain.clone();
        let port = input.port.clone();
        let headers = input.headers.clone();
        let body = req_body.clone();
        request_threads.push(std::thread::spawn(move ||{
            tls_post_req(&path,&domain,&port,&body,&headers,&reqs_p_conn);               
        }));
    }
    request_threads
}
