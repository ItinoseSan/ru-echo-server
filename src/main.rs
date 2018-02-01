extern crate time as crateTime;
extern crate colored;
use colored::*;

use std::net::{TcpListener, TcpStream};
use std::thread;

// traits
use std::io::Read;
use std::io::Write;

const LISTEN_PORT:u32 = 4500;

fn init_echo_server(){
    println!("Starting echo server...");
    println!("{}","Rust echo server started!".green());
    println!("Usage");
    println!("telnet localhost [SET_PORT_NUMBER]");
    set_listen_tcp_port(LISTEN_PORT);
} 
fn set_listen_tcp_port(port: u32){
    let address = format!("127.0.0.1:{}",port);
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("{}{}","failed:".red().bold(),e) }
            Ok(stream) => { println!("{}{:?}","New listening started:".blue().bold(),listener);
                thread::spawn(move || { handle_client(stream) });
            }
        }
    }
}
fn handle_client(mut stream: TcpStream) {
    let mut buf; 
    loop {
        // 受信バッファサイズのクリア化  
        buf = [0;512];
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
              m
            },
        };
        let data = stream.write(&buf);
        match data{
            Err(_) => break,
            Ok(_) => { 
                println!("{}{:?}","Server received data:",data);
                let now = crateTime::now();
                println!("{:?}{}",
                crateTime::strftime("%Y-%m-%d %H:%M:%S", &now).unwrap(),
                ":Echo server respond!".green().bold()
                );
            },        
        }
    }
}

fn main() {
    init_echo_server();
}