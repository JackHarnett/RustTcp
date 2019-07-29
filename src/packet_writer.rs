use crate::endian::Endianess::*;
use crate::endian::Endianess;

pub struct PacketWriter {
    pub data : Vec<u8>,
}


impl PacketWriter {

    pub fn new(data : Vec<u8>) -> PacketWriter {
        PacketWriter {
            data,
        }
    }

    // Reads
    pub fn write_u16(&mut self, endian : Endianess, int : u16)  {
        let first : u8 = (int >> 8) as u8;
        let second= (int - ((int >> 8) << 8)) as u8;

        println!("Written {}", int);
        self.data.append(&mut vec![first, second])
    }


}