pub struct DCPPacket {
    pub frameid: u16,
    pub serviceid: u8,
    pub service_type: u8,
    pub xid: u32,
    pub response_delay: u16, 
    pub length: u32,   
    pub payload: Vec<u8>,   
}
impl DCPPacket {
    pub fn new(
        frameid: u16,
        serviceid: u8,
        service_type: u8,
        xid: u32,
        response_delay: u16,        
        payload: Vec<u8>,
        
    ) -> DCPPacket {
        
        
        let length = payload.len() as u32;     
        DCPPacket {
            frameid: frameid,
            serviceid: serviceid,
            service_type: service_type,
            xid: xid,
            response_delay: response_delay,
            length: length,
            payload: payload,
            
        }
    }
    pub fn parse(data: &[u8]) -> DCPPacket {
        let frameid = u16::from_be_bytes([data[0], data[1]]);
        let serviceid = data[2];
        let servicetype = data[3];
        let xid = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        //response delay
        let response_delay = u16::from_be_bytes([data[8], data[9]]);
        //length        
        let length = u16::from_be_bytes([data[10], data[11]]);
        let payload = &data[12..];
        
        DCPPacket {
            frameid: frameid,
            serviceid: serviceid,
            service_type: servicetype,
            xid: xid,
            response_delay: response_delay,
            length : length as u32,
            payload: payload.to_vec(),
           
        }
    }
    pub fn validate(&self) -> bool {
        self.service_type == 1// Return true if service_type equals 1, otherwise return false
    }
    pub fn compile(&self) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();
        let mut header: Vec<u8> = Vec::new();

        header.extend_from_slice(&self.frameid.to_be_bytes()); //2
        header.push(self.serviceid); //1
        header.push(self.service_type); //1
        header.extend_from_slice(&self.xid.to_be_bytes()); //4
        header.extend_from_slice(&self.response_delay.to_be_bytes()); //2
        let length = self.payload.len() as u16;
       
        header.extend_from_slice(&length.to_be_bytes()); //2        
        packet.extend(&header);
        packet.extend(self.payload.clone());

        return packet;
    }
}
