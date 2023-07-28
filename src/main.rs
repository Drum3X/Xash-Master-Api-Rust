
mod parser;
mod connection;

use std::net::SocketAddr;
use tokio::time::Duration;
use futures::future::join_all;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "135.181.76.187:27010".parse().expect("Invalid Address");
    let iplist = match connection::get_iplist(addr, "valve", false, Duration::from_millis(500)).await {
        Ok(result) => result,
        Err(..) => todo!()
    };
    
    let mut processes = Vec::new();
    
    for addr in iplist {
        processes.push(connection::get_server_info(addr, Duration::from_millis(500)))
    }
    
    let result = join_all(processes).await;
    let mut i = 1;
    for server in result {
        match server {
            Ok(server) => {
                println!("{} {:?}\n\n", i.to_string(), server);
                i += 1; 
            }
            Err(..) => continue
        };
    }
}
