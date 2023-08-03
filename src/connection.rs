use crate::parser;

use format_bytes::format_bytes;
use std::net::SocketAddr;
use std::error::Error;
use tokio::net::UdpSocket;
use tokio::time;

pub async fn send_packet(
    packet: &[u8],
    addr: SocketAddr,
    timeout: time::Duration
) -> Result<Vec<u8>, Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    
    socket.connect(addr).await?;
    socket.send(packet).await?;

    let mut buffer: Vec<u8> = vec![0; 512];
    time::timeout(timeout, socket.recv(&mut buffer)).await??;

    Ok(buffer)
}

pub async fn get_iplist(
    addr: SocketAddr, 
    gamedir: &str, 
    nat: bool, 
    timeout: time::Duration
) -> Result<Vec<SocketAddr>, Box<dyn Error>> {
    let packet: Vec<u8> = format_bytes!(
        b"1\xff0.0.0.0:0\x00\\nat\\{}\\gamedir\\{}\\clver\\0.19.2\x00",
        match nat {
            true => b"1",
            false => b"0"
        },
        gamedir.as_bytes()
    );
    let result: Vec<u8> = send_packet(&packet, addr, timeout).await?;
    Ok(
        match parser::parse_master_info(result) {
            Ok(result) => result,
            Err(..) => return Err("Parsing error.".into())
        }
    )
}

pub async fn get_server_info(
    addr: SocketAddr, 
    timeout: time::Duration
) -> Result<parser::Server, Box<dyn Error>> {
    let packet: &[u8] = b"\xff\xff\xff\xffTSource";
    let result: Vec<u8> = send_packet(packet, addr, timeout).await?;
    Ok(
        match parser::parse_server_info(result) {
            Ok(mut result) => {
                result.ip = addr.ip().to_string();
                result.port = addr.port();
                result
            },
            Err(..) => return Err("Parsing error".into())
        }
    )
}