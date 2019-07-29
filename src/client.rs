use std::net::{TcpListener, TcpStream};
use std::fmt;
use std::io;
use std::io::{Read, Write};
use std::mem;

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

}