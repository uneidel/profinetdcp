use pnet::datalink::Channel;
use pnet::datalink::DataLinkReceiver;
use pnet::datalink::DataLinkSender;
use pnet::packet::ethernet::EtherType;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::Packet;

use regex::Regex;
use std::time::{Duration, Instant};

use crate::constants;

use crate::constants::ServiceID;
use crate::dcpblock::DCPBlock;
use crate::dcpblockrequest::*;
use crate::dcppaket::*;
use crate::utils::*;
use rand::RngCore;
use regex;
use std::fmt;

pub struct Dcphandler {
    pub _xid: u32,
    pub src_mac: String,
    pub sender: Box<dyn DataLinkSender>,
    pub receiver: Box<dyn DataLinkReceiver>,
}

impl Dcphandler {
    pub fn new(networkdevicename: String) -> Self {
        let interface = pnet::datalink::interfaces()
            .into_iter()
            .find(|iface| iface.name == networkdevicename)
            .unwrap();

        let (sender, receiver) = match pnet::datalink::channel(&interface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unknown channel type"),
            Err(e) => panic!("Error happened {}", e),
        };

        let mut rng = rand::thread_rng();
        let xid = rng.next_u32();
        let _source_mac = interface.mac.unwrap();

        Dcphandler {
            _xid: xid,
            src_mac: _source_mac.to_string(),
            sender,
            receiver,
        }
    }
    pub fn identify(&mut self, mac: &str) -> Option<Device> {
        let (option, suboption) = constants::Option::ALL;
        self.send_request(
            mac,
            constants::FrameID::IDENTIFY_REQUEST,
            constants::ServiceID::IDENTIFY,
            option,
            suboption,
            None,
            constants::RESPONSE_DELAY,
        );
        let start_time = Instant::now();

        while start_time.elapsed() < Duration::from_secs(2) {
            let result = self.read_response(false);
            match result {
                RequestResult::Device(d) => return Some(d),
                RequestResult::None => {
                    continue;
                }
                RequestResult::String(_s) => {}
            }
        }
        return None;
    }

    pub fn factory_reset(&mut self, mac: &str) {
        let (option, suboption) = constants::Option::RESET_FACTORY;
        let mut value: Vec<u8> = Vec::new();
        value.extend(constants::BlockQualifier::RESERVED);

        self.send_request(
            mac,
            constants::FrameID::GET_SET,
            ServiceID::SET,
            option,
            suboption,
            Some(value),
            constants::RESPONSE_DELAY,
        )
    }

    pub fn identify_all(&mut self) -> Vec<Device> {
        let dsc_mac = constants::PROFINET_MULTICAST_MAC_IDENTIFY;
        let (option, suboption) = constants::Option::ALL;
        self.send_request(
            dsc_mac,
            constants::FrameID::IDENTIFY_REQUEST,
            constants::ServiceID::IDENTIFY,
            option,
            suboption,
            None,
            constants::RESPONSE_DELAY,
        );
        let mut devices: Vec<Device> = Vec::new();
        let start_time = Instant::now();

        while start_time.elapsed() < Duration::from_secs(2) {
            let result = self.read_response(false);
            match result {
                RequestResult::Device(d) => devices.push(d),
                RequestResult::None => {
                    continue;
                }
                RequestResult::String(_s) => {}
            }
        }
        return devices;
    }

    pub fn blink(&mut self, mac: String) -> String {
        let mut value: Vec<u8> = Vec::new();
        value.extend(constants::BlockQualifier::RESERVED);
        value.extend(constants::LED_BLINK_VALUE);
        let (option, suboption) = constants::Option::BLINK_LED;
        self.send_request(
            mac.as_str(),
            constants::FrameID::GET_SET,
            constants::ServiceID::SET,
            option,
            suboption,
            Some(value),
            7,
        );

        let start_time = Instant::now();
        let mut res = String::new();
        while start_time.elapsed() < Duration::from_secs(2) {
            let result = self.read_response(true);
            res = match result {
                RequestResult::Device(_d) => {
                    continue;
                }
                RequestResult::None => {
                    continue;
                }
                RequestResult::String(s) => s,
            };
        }
        return res;
    }

    pub fn set_name_of_station(&mut self, mac: String, name: String) -> String {
        let pattern = Regex::new(r"^[a-z][a-zA-Z0-9\-.]*$").unwrap();
        if !pattern.is_match(name.clone().as_str()) {
            return "Invalid Pattern".to_string();
        }
        let stationname = name.to_lowercase();
        let mut value: Vec<u8> = Vec::new();
        value.extend(constants::BlockQualifier::STORE_PERMANENT);
        value.extend(stationname.as_bytes());
        let (option, suboption) = constants::Option::NAME_OF_STATION;

        self.send_request(
            mac.as_str(),
            constants::FrameID::GET_SET,
            constants::ServiceID::SET,
            option,
            suboption,
            Some(value),
            7,
        );

        let start_time = Instant::now();
        let mut res = String::new();
        while start_time.elapsed() < Duration::from_secs(2) {
            let result = self.read_response(true);
            res = match result {
                RequestResult::Device(_d) => {
                    continue;
                }
                RequestResult::None => {
                    continue;
                }
                RequestResult::String(s) => s,
            };
        }
        return res;
    }

    fn send_request(
        &mut self,
        dst_mac: &str,
        frame_id: u16,
        service_id: u8,
        option: u8,
        suboption: u8,
        valuepayload: Option<Vec<u8>>,
        response_delay: u16,
    ) {
        let xid = &self._xid + 1;
        let payload = match valuepayload {
            Some(val) => val,
            None => Vec::new(),
        };
        let p = payload.clone();
        let dcpblockrequestpayload = DCPBlockRequest::new(option, suboption, p).compile();
        let servicetype = constants::ServiceType::REQUEST; // Request

        let dcppacket = DCPPacket::new(
            frame_id,
            service_id,
            servicetype,
            xid,
            response_delay,
            dcpblockrequestpayload,
        );
        let p = dcppacket.compile();
        let dcppacketpayload = p.as_slice();

        let smac = mac_address_string_to_bytes(self.src_mac.as_str());
        let sourcemac =
            pnet::datalink::MacAddr::new(smac[0], smac[1], smac[2], smac[3], smac[4], smac[5]);
        let tmac = mac_address_string_to_bytes(dst_mac);
        let targetmac =
            pnet::datalink::MacAddr::new(tmac[0], tmac[1], tmac[2], tmac[3], tmac[4], tmac[5]);

        let mut ethernet_buffer = [0u8; 42];
        let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();
        ethernet_packet.set_destination(targetmac);
        ethernet_packet.set_source(sourcemac);
        ethernet_packet.set_ethertype(EtherType(constants::ETHER_TYPE));
        ethernet_packet.set_payload(dcppacketpayload);

        self.sender
            .send_to(ethernet_packet.packet(), None)
            .unwrap()
            .unwrap();
    }

    fn read_response(&mut self, setrequest: bool) -> RequestResult {
        let mut device: Device = Device::default();
        let buf = self.receiver.next().unwrap();

        let p = pnet::packet::ethernet::EthernetPacket::new(buf).unwrap();
        let dcppaket = DCPPacket::parse(p.payload());

        if dcppaket.validate() == false {
            return RequestResult::None;
        }
        // we need to check if its a valid dcp paket
        let mut blocks = dcppaket.payload;

        if setrequest == true && blocks[0] == 5 {
            let response = get_responsecode(blocks[6] as i32);
            return RequestResult::String(response);
        }
        let mut length = dcppaket.length as usize;

        let mac = p.get_source().to_string();
        device.mac = mac;
        while length > 6 {
            let len = self.process_blocks(&blocks, &mut device);
            blocks = blocks.split_off(len + 4);
            length = length - (4 + len);
        }
        return RequestResult::Device(device);
    }

    fn process_blocks(&self, blocks: &Vec<u8>, device: &mut Device) -> usize {
        let dcpblock = DCPBlock::parse(blocks.as_slice());

        if dcpblock.option == 2 && dcpblock.suboption == 2 {
            let nameofstation = dcpblock.payload;
            let ns = String::from_utf8(nameofstation).expect("Invalid UTF-8");
            device.nameofstation = ns.clone();
        } else if dcpblock.option == 1 && dcpblock.suboption == 2 {
            device.ip = dotted_decimal_bytes_to_string(&dcpblock.payload[2..6]);
            device.netmask = dotted_decimal_bytes_to_string(&dcpblock.payload[6..10]);
            device.gateway = dotted_decimal_bytes_to_string(&dcpblock.payload[10..14]);
        } else if dcpblock.option == 2 && dcpblock.suboption == 1 {
            let mut device_family = dcpblock.payload;
            while let Some(b'\x00') = device_family.last() {
                device_family.pop();
            }
            let device_family = String::from_utf8(device_family).expect("Invalid UTF-8");
            device.family = device_family;
        }

        let blocklen = (dcpblock.length as usize) + (dcpblock.length as usize % 2);
        return blocklen;
    }
}

enum RequestResult {
    Device(Device),
    String(String),
    None,
}
fn get_responsecode(choice: i32) -> String {
    let result = match choice {
        0 => "Code 00: Set successful",
        1 => "Code 01: Option unsupported",
        2 => "Code 02: Suboption unsupported or no DataSet available",
        3 => "Code 03: Suboption not set",
        4 => "Code 04: Resource Error",
        5 => "Code 05: SET not possible by local reasons",
        6 => "Code 06: In operation, SET not possible",
        _ => "unknown",
    };
    return result.to_string();
}
pub struct Device {
    pub ip: String,
    pub mac: String,
    pub netmask: String,
    pub gateway: String,
    pub family: String,
    pub nameofstation: String,
}
// Implement the Display trait for MyStruct
impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the formatted data to the provided formatter
        write!(
            f,
            "{{
            IP: {}
            MAC: {}
            Netmask: {}
            Gateway: {}
            Family: {}
            NameofStation: {}
        }}",
            self.ip, self.mac, self.netmask, self.gateway, self.family, self.nameofstation
        )
    }
}
impl Default for Device {
    fn default() -> Self {
        Self {
            ip: String::new(),
            mac: String::new(),
            netmask: String::new(),
            gateway: String::new(),
            family: String::new(),
            nameofstation: String::new(),
        }
    }
}
