use std::net::{TcpListener, TcpStream};
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::io::{Read, Write};
use std::str;
use std::mem;
use std::thread;
use crate::packet::Packet;


pub struct Client {
    pub stream : TcpStream,
}

impl Client {

    /// Creates a connection to the server
    pub fn connect(server : String, port : i32) -> Result<TcpStream, io::Error> {
        let host = format!("{}:{}", server, port);
        TcpStream::connect(host)
    }

    /// Writes the given byte array to the connection stream
    pub fn write(&mut self, msg : &[u8]) {
        self.stream.write(msg);
    }

    /// Writes 50 bytes starting with the given bytes to the stream.
    /// Pads the tail of the bytes to make up to 50.
    pub fn write_block(&mut self, msg : &[u8]) {
        let size = mem::size_of_val(msg);
        let mut data : [u8; 50] = [0 as u8; 50];

        let mut vec_msg = Vec::from(msg);
        let mut vec_dat = vec![0 as u8; 50-size];

        vec_msg.append(&mut vec_dat);

        for (idx, ele) in data.iter_mut().enumerate() {
            *ele = vec_msg[idx];
        }

        self.write(&data);
    }

    pub fn read(&mut self) {
        let reader = BufReader::new(&self.stream);
    }

    pub fn write_packet(&mut self, packet : &Packet) {
        self.write(&[packet.opcode]);
        self.write(&[packet.size]);
        self.write(&packet.payload);
    }

    pub fn listen(&mut self) -> Packet {
        loop {
            let mut reader = BufReader::new(&self.stream);

            let mut opcode: [u8; 1] = [0 as u8];
            let mut size: [u8; 1] = [0 as u8];

            reader.read_exact(&mut opcode);
            reader.read_exact(&mut size);

            let mut vec_data = vec![0 as u8; size[0] as usize];

            reader.read_exact(&mut vec_data);

            println!("Got packet {} of size {}", opcode[0], size[0]);

            return Packet::new(opcode[0], vec_data)
        }
    }

    pub fn handle_packet_10(&self, data : Vec<u8>) {

    }
}