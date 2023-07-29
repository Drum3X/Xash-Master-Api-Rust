
mod parser;
mod connection;

use std::net::SocketAddr;
use tokio::time::Duration;
use futures::future::join_all;

async fn get_servers() -> Vec<parser::Server>{
    let addr: SocketAddr = "135.181.76.187:27010".parse().expect("Invalid Address");
    let iplist = match connection::get_iplist(addr, "cstrike", false, Duration::from_millis(500)).await {
        Ok(result) => result,
        Err(..) => todo!()
    };
    
    let mut processes = Vec::new();
    
    for addr in iplist {
        processes.push(connection::get_server_info(addr, Duration::from_millis(250)))
    }
    
    let result = join_all(processes).await;
    let mut servers = Vec::new();
    for server in result {
        match server {
            Ok(server) => servers.push(server),
            Err(..) => continue
        }
    }
    
    servers
}

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    let servers = get_servers().await;
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("89.163.130.135", 8000))?
    .run()
    .await
}
