use std::mem;

pub struct Packet {
    pub opcode : u8,
    pub size : u8,
    pub payload : Vec<u8>,
}

impl Packet {

    pub fn new(opcode : u8, payload : Vec<u8>) -> Packet {
        Packet {
            opcode,
            size : payload.len() as u8,
            payload : payload
        }
    }

}