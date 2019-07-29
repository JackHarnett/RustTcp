use std::mem;

pub struct Packet {
    opcode : u16,
    payload : [u8],
}

impl Packet {

    fn num_blocks(&mut self) {
    }

}