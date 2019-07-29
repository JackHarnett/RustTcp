use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{Read, Write};
use std::fmt;
use std::thread;
use std::str;

pub struct Server {
    pub listener : TcpListener,
}

impl Server {

    pub fn bind(host : String, port : u32) -> Result<TcpListener, io::Error> {
        println!("Starting server on {}:{}", host, port);
        TcpListener::bind(format!("{}:{}", host, port))
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {

            match(stream) {
                Ok(mut client) => {
                    println!("New client connected from {}", client.local_addr().unwrap());

                    thread::spawn(move|| {let mut data = [0 as u8; 50];

                        while match client.read(&mut data) {
                            Ok(size) => {
                                let data = str::from_utf8(&data[0..50]).unwrap();

                                if !data.is_empty() {
                                    println!("Got packet: {}", data);
                                }

                                true
                            },

                            Err(err) => {
                                println!("Error reading data {}", err);

                                false
                            }
                        } {}
                    });
                }

                Err(err) => println!("Error creating client! {}", err)
            }

            // Create a new thread per connection
        }
    }

}