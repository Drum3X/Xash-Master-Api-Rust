
mod unpack;

use std::net::SocketAddr;
use std::error::Error;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Server {
    pub ip: String,
    pub port: u16,
    
    //old engine info
    pub engine_type: char,
    pub protocol_ver: u8,
    pub hostname: String,
    pub map: String,
    pub gamedir: String,
    pub gamedesc: String,
    pub appid: i16,
    pub numplayers: u8,
    pub maxplayers: u8,
    pub numbots: u8,
    pub dedicated: u8,
    pub os: String,
    pub passworded: u8,
    pub secure: u8,
    
    //new engine extra info
    pub address: String,
    pub servertype: char,
    pub is_mod: u8,
    pub game_url: String,
    pub update_url: String,
    pub null: u8,
    pub mod_ver: i32,
    pub mod_size: i32,
    pub mod_type: u8,
    pub dll_type: u8,
    pub bots: u8
}

pub fn parse_master_info(data: Vec<u8>) -> Result<Vec<SocketAddr>, Box<dyn Error>> {
    let mut data: Vec<u8> = data[6..].to_vec();
    
    let mut servers: Vec<SocketAddr> = Vec::new();
    while !data.is_empty() && !(data[0] == 0) {
        servers.push(
            format!(
                "{}.{}.{}.{}:{}", 
                unpack::unpack_u8(&mut data), 
                unpack::unpack_u8(&mut data), 
                unpack::unpack_u8(&mut data), 
                unpack::unpack_u8(&mut data), 
                unpack::unpack_u16_be(&mut data)
            ).parse::<SocketAddr>().expect("Invalid Address")
        )
    }
    
    Ok(servers)
}

pub fn parse_server_info(data: Vec<u8>) -> Result<Server, Box<dyn Error>> {
    let mut data: Vec<u8> = data;
    let mut server: Server = Server::default(); 
    
    //if the master response fails
    if unpack::unpack_i32(&mut data) != -1 {
        return Err("Invalid response.".into());
    }
    
    server.engine_type = unpack::unpack_u8(&mut data) as char;
    
    if server.engine_type == 'I' {
        //old engine 
        server.protocol_ver = unpack::unpack_u8(&mut data);
        server.hostname = unpack::unpack_string(&mut data);
        server.map = unpack::unpack_string(&mut data); 
        server.gamedir = unpack::unpack_string(&mut data);
        server.gamedesc = unpack::unpack_string(&mut data);
        server.appid = unpack::unpack_i16(&mut data);
        server.numplayers = unpack::unpack_u8(&mut data);
        server.maxplayers = unpack::unpack_u8(&mut data);
        server.numbots = unpack::unpack_u8(&mut data);
        server.dedicated = unpack::unpack_u8(&mut data);
        server.os = match unpack::unpack_u8(&mut data) as char {
            'L' | 'l' => String::from("Linux"),
            'W' | 'w' => String::from("Windows"),
            'M' | 'm' => String::from("Mac OS"),
            _ => String::from("Unknown OS")
        };
        server.passworded = unpack::unpack_u8(&mut data);
        server.secure = unpack::unpack_u8(&mut data);
    } else if server.engine_type == 'm' {
        //new engine 
        server.address = unpack::unpack_string(&mut data);
        server.hostname = unpack::unpack_string(&mut data);
        server.map = unpack::unpack_string(&mut data);
        server.gamedir = unpack::unpack_string(&mut data);
        server.gamedesc = unpack::unpack_string(&mut data);
        server.numplayers = unpack::unpack_u8(&mut data);
        server.maxplayers = unpack::unpack_u8(&mut data);
        server.protocol_ver = unpack::unpack_u8(&mut data);
        server.servertype = unpack::unpack_u8(&mut data) as char;
        server.os = match unpack::unpack_u8(&mut data) as char {
            'L' | 'l' => String::from("Linux"),
            'W' | 'w' => String::from("Windows"),
            'M' | 'm' => String::from("Mac OS"),
            _ => String::from("Unknown OS")
        };
        server.is_mod = unpack::unpack_u8(&mut data);
        
        if server.is_mod == 1 {
            server.game_url = unpack::unpack_string(&mut data);
            server.update_url = unpack::unpack_string(&mut data);
            server.null = unpack::unpack_u8(&mut data);
            server.mod_ver = unpack::unpack_i32(&mut data);
            server.mod_size = unpack::unpack_i32(&mut data);
            server.mod_type = unpack::unpack_u8(&mut data);
            server.dll_type = unpack::unpack_u8(&mut data);
        }
        
        server.secure = unpack::unpack_u8(&mut data);
        server.bots = unpack::unpack_u8(&mut data);
    } else {
        return Err("Invalid engine type.".into());
    }
    
    Ok(server)
}