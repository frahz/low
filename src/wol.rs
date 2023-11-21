use crate::macaddr::MacAddress;
use std::net::{SocketAddr, UdpSocket};

pub const WOL_HEADER: [u8; 6] = [0xFF; 6];
pub const HEADER_OFFSET: usize = 0;
pub const WOL_LENGTH: usize = HEADER_OFFSET + (6 + 6 * 16);

pub struct WolPacket(pub [u8; WOL_LENGTH]);

impl WolPacket {
    pub fn create(mac_address: &MacAddress) -> Self {
        let mut bytes = [0; WOL_LENGTH];
        // let hw_addr = MacAddress::get("eth0").unwrap();
        // bytes[..6].copy_from_slice(&mac_address.bytes);
        // bytes[6..12].copy_from_slice(&hw_addr.bytes);
        // bytes[12] = 0x08;
        // bytes[13] = 0x42;

        for (idx, b) in WOL_HEADER.iter().enumerate() {
            bytes[idx + HEADER_OFFSET] = *b;
            for offset in 1..17 {
                bytes[idx + HEADER_OFFSET + 6 * offset] = mac_address.bytes[idx];
            }
        }

        Self(bytes)
    }
}

pub fn create_socket(target_ip: &str) -> Result<UdpSocket, String> {
    let src_addrs = [
        SocketAddr::from(([0, 0, 0, 0], 0000)),
        // SocketAddr::from(([0, 0, 0, 0], 9101)),
        // SocketAddr::from(([0, 0, 0, 0], 9102)),
    ];

    let socket = match UdpSocket::bind(&src_addrs[..]) {
        Ok(s) => s,
        Err(e) => return Err(format!("Couldn't bind address error: {e}")),
    };

    if let Err(e) = socket.set_broadcast(true) {
        return Err(format!("Couldn't set broadcast error: {e}"));
    };

    if let Err(e) = socket.connect(target_ip) {
        return Err(format!("Couldn't connect to IP: {target_ip} error: {e}"));
    }

    Ok(socket)
}
