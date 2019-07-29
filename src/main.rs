use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{Read, Write};
use std::thread;
use std::time;
use std::fmt;
use std::str;

mod client;
mod server;
mod packet;
mod packet_reader;
mod packet_writer;
mod endian;

use server::Server;
use client::Client;
use crate::packet::Packet;
use crate::packet_writer::PacketWriter;
use crate::endian::Endianess::BIG;

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

    let mut client1 = Client {stream : Client::connect("127.0.0.1".to_string(), 18000).unwrap()};
    let mut client2 = Client {stream : Client::connect("127.0.0.1".to_string(), 18000).unwrap()};

    let hello = Packet::new(10, b"Hello World!".to_vec());
    let world = Packet::new(10, b"Hello World from me!".to_vec());
    client1.write_packet(&hello);

    let us : u16 = 15;

    let num = vec![100, 100 as u8];

    let mut writer = PacketWriter::new(vec![]);
    writer.write_u16(BIG, 258);

    let number2 = Packet::new(1, writer.data);

    client2.write_packet(&number2);

    server.join();
    Ok(())
}
