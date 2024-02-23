#![allow(dead_code)]
// The multicast address for identifying all requests
pub const PROFINET_MULTICAST_MAC_IDENTIFY: &str = "01:0e:cf:00:00:00";
// The response delay value for DCP requests
pub const RESPONSE_DELAY: u16 = 0x0080;
// Ether type of DCP packets
pub const ETHER_TYPE: u16 = 0x8892;
// Value for letting the LED blink
pub const LED_BLINK_VALUE: [u8; 2] = [0x01, 0x00];

pub struct FrameID;

impl FrameID {
    // Constants for the different DCP frame IDs
    pub const GET_SET: u16 = 0xfefd;
    pub const IDENTIFY_REQUEST: u16 = 0xfefe;
}

pub struct BlockQualifier;

impl BlockQualifier {
    // DCP block qualifiers
    // Indicate that a value should be stored permanently
    pub const STORE_PERMANENT: [u8; 2] = [0x00, 0x01];
    // Reset to factory with mode communication
    pub const RESET_COMMUNICATION: [u8; 2] = [0x00, 0x04];
    // Indicate that a value should be stored temporarily (value will be discarded after power reset)
    pub const STORE_TEMPORARY: [u8; 2] = [0x00, 0x00];
    // Used for ControlOption with Suboption other than SuboptionResetToFactory (eg. when flashing LED)
    pub const RESERVED: [u8; 2] = [0x00, 0x00];
}

struct ResetFactoryModes;

impl ResetFactoryModes {
    // Reset to factory modes
    const RESET_APPLICATION_DATA: [u8; 2] = [0x00, 0x02];
    const RESET_COMMUNICATION: [u8; 2] = [0x00, 0x04];
    const RESET_ENGINEERING: [u8; 2] = [0x00, 0x06];
    const RESET_ALL_DATA: [u8; 2] = [0x00, 0x8];
    const RESET_DEVICE: [u8; 2] = [0x00, 0x10];
    const RESET_AND_RESTORE: [u8; 2] = [0x00, 0x12];
}

pub struct ServiceType;

impl ServiceType {
    // Service type of a DCP packet
    pub const REQUEST: u8 = 0;
    pub const RESPONSE: u8 = 1;
}

pub struct ServiceID;

impl ServiceID {
    // Service ID of a DCP packet
    pub const GET: u8 = 3;
    pub const SET: u8 = 4;
    pub const IDENTIFY: u8 = 5;
}

pub struct Option;

impl Option {
    // Option and suboption pairs for DCP blocks
    pub const IP_ADDRESS: (u8, u8) = (1, 2);
    pub const DEVICE_FAMILY: (u8, u8) = (2, 1);
    pub const NAME_OF_STATION: (u8, u8) = (2, 2);
    pub const DEVICE_ID: (u8, u8) = (2, 3);
    pub const BLINK_LED: (u8, u8) = (5, 3);
    pub const RESET_FACTORY: (u8, u8) = (5, 5);
    pub const RESET_TO_FACTORY: (u8, u8) = (5, 6);
    pub const ALL: (u8, u8) = (0xFF, 0xFF);
}
