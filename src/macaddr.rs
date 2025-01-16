use nix::ifaddrs::*;
use std::{num, result};
use thiserror::Error;

#[derive(Debug)]
pub struct MacAddress {
    pub bytes: [u8; 6],
}

#[derive(Error, Debug)]
pub enum MacAddressError {
    #[error("Invalid Mac Address length: {0}")]
    InvalidLength(usize),

    #[error("Couldn't parse Mac Address: {mac_address}")]
    ParseError {
        mac_address: String,
        cause: num::ParseIntError,
    },

    #[error("Couldn't get interface addresses")]
    Errno(#[from] nix::errno::Errno),
}

type Result<T> = result::Result<T, MacAddressError>;

impl MacAddress {
    pub fn parse(address: &str) -> Result<Self> {
        let mac = address.replace(&[':', '.', '-'], "");
        let mut mac_bytes = [0; 6];

        if mac.len() != 12 {
            return Err(MacAddressError::InvalidLength(mac.len()));
        }

        for (idx, byte) in mac_bytes.iter_mut().enumerate() {
            let offset = 2 * idx;
            *byte = match u8::from_str_radix(&mac[offset..offset + 2], 16) {
                Ok(x) => x,
                Err(e) => {
                    return Err(MacAddressError::ParseError {
                        mac_address: address.to_string(),
                        cause: e,
                    })
                }
            }
        }

        Ok(Self { bytes: mac_bytes })
    }

    pub fn get(ifname: &str) -> Result<Self> {
        let iflist = getifaddrs()?;

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
