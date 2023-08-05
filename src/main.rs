
mod parser;
mod connection;

use actix_web::{App, HttpResponse, HttpServer, Responder, web, get};
use tokio::time::Duration; 
use std::net::SocketAddr;
use futures::future::join_all;
use regex::RegexBuilder;
use serde::Deserialize;

//if you have your own master you can change it
const MASTER_ADDR: &str = "135.181.76.187:27010";

//API query options
#[derive(Deserialize)]
struct QueryOptions {
    gamedir: Option<String>,
    nat: Option<bool>,
    ip: Option<String>,
    port: Option<u16>,
    search: Option<String>,
    master_timeout: Option<u64>,
    server_timeout: Option<u64>
}

#[get("/")]
async fn index(info: web::Query<QueryOptions>) -> impl Responder {
    let addr: SocketAddr = MASTER_ADDR.parse().unwrap();
    
    //API queries
    let gamedir: String = match &info.gamedir {
        Some(gamedir) => gamedir.clone(),
        None => String::from("Valve")
    };
    
    let nat: bool = match info.nat {
        Some(nat) => nat,
        None => false
    };
    
    let master_timeout: u64 = match info.master_timeout {
        Some(timeout) => timeout,
        None => 500
    };
    
    let server_timeout: u64 = match info.server_timeout {
        Some(timeout) => timeout,
        None => 250
    };
    
    let iplist: Vec<SocketAddr> = match connection::get_iplist(addr, &gamedir, nat, Duration::from_millis(master_timeout)).await {
        Ok(result) => result,
        Err(..) => return HttpResponse::Ok().body("[]")
    };
    
    let mut processes = Vec::new();
    
    for addr in iplist {
        processes.push(connection::get_server_info(addr, Duration::from_millis(server_timeout)))
    }
    
    let result = join_all(processes).await;
    
    let mut servers = Vec::new();
    for server in result {
        match server {
            Ok(server) => { 
                //API queries 
                match &info.ip {
                    Some(ip) => {
                        if ip.clone() != server.ip {
                            continue
                        }
                    },
                    None => {}
                }; 
                
                match info.port {
                    Some(port) => {
                        if port != server.port {
                            continue
                        }
                    },
                    None => {}
                };
                
                match &info.search {
                    Some(search) => {
                        match RegexBuilder::new(&search.clone())
                            .case_insensitive(true)
                            .build() {
                            Ok(re) => {
                                if !re.is_match(&server.hostname) {
                                    continue
                                }
                            }
                            Err(..) => return HttpResponse::Ok().body("[]")
                        }
                    },
                    None => {}
                }; 
                
                servers.push(server)
            },
            Err(..) => continue
        }
    } 
    
    HttpResponse::Ok().json(servers)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
