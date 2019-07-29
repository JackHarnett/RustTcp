use crate::endian::Endianess::*;
use crate::endian::Endianess;

pub struct PacketReader {
    pub data : Vec<u8>,
    pub current : usize,
}

impl PacketReader {

    pub fn new(data : Vec<u8>) -> PacketReader {
        PacketReader {
            data,
            current : 0
        }
    }

    // Reads
    pub fn read_u16(&mut self, endian : Endianess) -> u16 {
        let first : u16 = self.data[self.current] as u16;
        let second : u16 = self.data[self.current + 1] as u16;

        if endian == LITTLE {
            return ((first << 8) + second) as u16;
        } else {
            return ((second << 8) + first) as u16;
        }
    }


}