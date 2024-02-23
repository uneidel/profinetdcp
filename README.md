# Profinet Discovery Protocol
Profinet DCP (Discovery and Configuration Protocol) is a fundamental aspect of Profinet communication within industrial automation systems. This protocol facilitates the discovery and (configuration) of Profinet devices within a network, allowing for seamless integration and interoperability.


Note:    
This need to run in privileged mode or you use [capabilities](https://linux.die.net/man/7/capabilities) 

### Example
```rust
fn main(){

    let networkdevicename = "wlp3s0".to_string();
    let mac = "00:00:00:00:00";
    let mut dcp = Dcphandler::new(networkdevicename);
    
    dcp.blink(mac.clone());
    dcp.set_name_of_station(mac.clone(), "cool".to_string());
    dcp.factory_reset(mac.clone());
    let devices = dcp.identify_all();
    for device in &devices {
        println!("Device: {}", device);
    } 
}
```

## credits
this library was inspired by https://gitlab.com/pyshacks/pnio_dcp/