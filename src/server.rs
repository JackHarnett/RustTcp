use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{Read, Write, Seek};
use std::io::BufReader;
use std::fmt;
use std::thread;
use std::str;
use crossbeam;
use std::sync::Arc;

pub struct Server {
    pub listener : TcpListener,
}

impl<'a> Server {

    pub fn bind(host : String, port : u32) -> Result<TcpListener, io::Error> {
        println!("Starting server on {}:{}", host, port);
        TcpListener::bind(format!("{}:{}", host, port))
    }

    pub fn listen(&mut self) {
        loop {
            for stream in self.listener.incoming() {
                let mut client = stream.unwrap();
                let mut reader = BufReader::new(client);

                thread::spawn(move|| {
                   loop {
                       let mut opcode: [u8; 1] = [0 as u8];
                       let mut size: [u8; 1] = [0 as u8];

                       reader.read_exact(&mut opcode);
                       reader.read_exact(&mut size);

                       let mut vec_data = vec![0 as u8; size[0] as usize];

                       reader.read_exact(&mut vec_data);

                       let str_data = str::from_utf8(&vec_data).unwrap();

                       println!("Got packet {} of size {} with data {}", opcode[0], size[0], str_data)
                   }
                });
            }
        }
    }

}