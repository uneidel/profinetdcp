pub struct DCPBlockRequest {
    pub option: u8,
    pub suboption: u8,
    pub payload: Vec<u8>,
}

impl DCPBlockRequest {
    pub fn new(option: u8, suboption: u8, payload: Vec<u8>) -> DCPBlockRequest {
        // build header

        return DCPBlockRequest {
            option: option,
            suboption: suboption,
            payload: payload,
        };
    }
    pub fn compile(&self) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::new();
        header.push(self.option); // unsigned char 1Byte
        header.push(self.suboption); // unsigned char 1Byte
        let paylen = self.payload.len() as u16; //unsigned short 2byte
        let mut pay = self.payload.clone();
        // if the payload has odd length, add one byte padding at the end
        if (paylen % 2) != 0 {
            pay.push(0x00);
        }
        header.extend_from_slice(&paylen.to_be_bytes());

        let mut packet: Vec<u8> = Vec::new();
        packet.extend(&header);
        packet.extend(&pay);

        return packet;
    }
}

