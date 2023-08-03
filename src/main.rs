
mod parser;
mod connection;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use tokio::time::Duration; 
use std::net::SocketAddr;
use futures::future::join_all;
use serde::Deserialize;

const MASTER_ADDR: &str = "135.181.76.187:27010";

#[derive(Deserialize)]
struct QueryInfo {
    gamedir: Option<String>,
    nat: Option<bool>,
    ip: Option<String>,
    port: Option<u16>,
}

#[get("/")]
async fn index(info: web::Query<QueryInfo>) -> impl Responder {
    let addr: SocketAddr = MASTER_ADDR.parse().expect("Invalid Address");
    
    let gamedir = match &info.gamedir {
        Some(gamedir) => gamedir.clone(),
        None => String::from("Valve")
    };
    
    let nat = match info.nat {
        Some(nat) => nat,
        None => false
    };
    
    let iplist = match connection::get_iplist(addr, &gamedir, nat, Duration::from_millis(500)).await {
        Ok(result) => result,
        Err(..) => return HttpResponse::Ok().body("[]")
    };
    
    let mut processes = Vec::new();
    
    for addr in iplist {
        processes.push(connection::get_server_info(addr, Duration::from_millis(250)))
    }
    
    let result = join_all(processes).await;
    
    let mut servers = Vec::new();
    for server in result {
        match server {
            Ok(server) => { 
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
