use nix::ifaddrs::*;

#[derive(Debug)]
pub struct MacAddress {
    pub bytes: [u8; 6],
}

impl MacAddress {
    pub fn parse(address: &str) -> Result<Self, String> {
        let mac = address.replace(&[':', '.', '-'], "");
        let mut mac_bytes = [0; 6];

        if mac.len() != 12 {
            return Err(format!(
                "Incorrect length provided for Mac Address len: {}",
                mac.len()
            ));
        }

        for (idx, byte) in mac_bytes.iter_mut().enumerate() {
            let offset = 2 * idx;
            *byte = match u8::from_str_radix(&mac[offset..offset + 2], 16) {
                Ok(x) => x,
                Err(_) => return Err("Couldn't parse Mac Address".to_string()),
            }
        }

        Ok(Self { bytes: mac_bytes })
    }

    pub fn get(ifname: &str) -> Result<Self, String> {
        let iflist = getifaddrs().unwrap();

        for interface in iflist {
            if let Some(address) = interface.address {
                if let Some(link) = address.as_link_addr() {
                    let bytes = link.addr();
                    if let Some(bytes) = bytes {
                        if ifname == interface.interface_name {
                            println!("Interface name: {}", interface.interface_name);
                            println!("Mac address: {:02x?}", bytes);
                            return Ok(Self { bytes });
                        }
                    }
                }
            }
        }

        Ok(Self { bytes: [0; 6] })
    }
}
