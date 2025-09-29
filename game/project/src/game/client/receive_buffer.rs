use crate::proto;
use protobuf::ClearAndParse;

pub struct ReceiveBuffer {
    recived_size_byte: u32,
    size: u32,
    body: Vec<u8>,
}

impl ReceiveBuffer {
    pub fn new() -> Self {
        ReceiveBuffer {
            recived_size_byte: 0,
            size: 0,
            body: Vec::new(),
        }
    }

    pub fn read_stream(&mut self, packet: &mut Vec<u8>) -> Vec<proto::Packet> {
        let mut result = Vec::new();

        while !packet.is_empty() {
            if !self.read_length_header(packet) {
                // ヘッダ読み込み中
                return result;
            }

            let read_size = std::cmp::min(self.size, packet.len() as u32) as usize;
            self.body.extend(packet.drain(0..read_size));
            self.size -= read_size as u32;

            if self.size == 0 {
                // パケット読み込み完了
                let mut temp = crate::proto::Packet::new();
                let res = temp.clear_and_parse(&self.body);
                if res.is_ok() {
                    result.push(temp);
                }
                self.body.clear();
                self.recived_size_byte = 0;
                self.size = 0;
            }
        }
        result
    }

    fn read_length_header(&mut self, packet: &mut Vec<u8>) -> bool {
        let sizeof_u32 = std::mem::size_of::<u32>() as u32;
        let sizeof_u8 = std::mem::size_of::<u8>() as u32;
        if self.recived_size_byte >= sizeof_u32 {
            return true;
        }

        let needed = sizeof_u32 - self.recived_size_byte;
        let to_read = std::cmp::min(needed, packet.len() as u32) as usize;
        self.size |= (packet
            .drain(0..to_read)
            .fold(0u32, |acc, b| (acc << 8) | b as u32))
            << (self.recived_size_byte * sizeof_u8);
        self.recived_size_byte += to_read as u32;

        return self.recived_size_byte >= sizeof_u32;
    }
}
