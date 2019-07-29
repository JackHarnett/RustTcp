use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{Read, Write};
use std::thread;
use std::time;
use std::fmt;
use std::str;

mod client;
mod server;

use server::Server;
use client::Client;

fn handle_client(mut stream : TcpStream) {
    println!("Stream: {}", stream.local_addr().unwrap());

    // Read data into a 100 byte array
    let mut data = [0 as u8; 50];

    while match(stream.read(&mut data)) {
        Ok(size) => {
            let data = str::from_utf8(&data[0..size]).unwrap();
            if(!data.is_empty()) {
                println!("Got {}", data);
            }
            true
        },

        Err(err) => {
            println!("Error {}", err);
            false
        },
    } {}
}

fn main() -> io::Result<()>{
    println!("Starting Rust Server");

    let server = thread::spawn(move || {
        //start_server()

        let mut server = Server { listener : Server::bind("127.0.0.1".to_string(), 18000).unwrap() };
        server.listen();
    });

    thread::sleep(time::Duration::from_millis(100));

    let mut client = Client {stream : Client::connect("127.0.0.1".to_string(), 18000).unwrap()};

    client.write_block(b"Hello Server");
    client.write_block(b"Hello Server 2");
    client.write_block(b"Hello Server 3");

    server.join();
    Ok(())
}
