use crate::macaddr::MacAddress;
use std::net::{SocketAddr, UdpSocket};
use std::{io, result};
use thiserror::Error;

pub const WOL_HEADER: [u8; 6] = [0xFF; 6];
pub const HEADER_OFFSET: usize = 0;
pub const WOL_LENGTH: usize = HEADER_OFFSET + (6 + 6 * 16);

pub struct WolPacket(pub [u8; WOL_LENGTH]);

#[derive(Error, Debug)]
pub enum SocketError {
    #[error("Couldn't bind to address: {cause}")]
    BindFailed { cause: io::Error },

    #[error("Couldn't set broadcast address: {cause}")]
    BroadcastFailed { cause: io::Error },

    #[error("Couldn't connect to IP: {target_ip} error: {cause}")]
    ConnectionFailed { target_ip: String, cause: io::Error },
}

type Result<T> = result::Result<T, SocketError>;

impl WolPacket {
    pub fn create(mac_address: &MacAddress) -> Self {
        let mut bytes = [0; WOL_LENGTH];

        for (idx, b) in WOL_HEADER.iter().enumerate() {
            bytes[idx + HEADER_OFFSET] = *b;
            for offset in 1..17 {
                bytes[idx + HEADER_OFFSET + 6 * offset] = mac_address.bytes[idx];
            }
        }

        Self(bytes)
    }
}

pub fn create_socket(target_ip: &str) -> Result<UdpSocket> {
    let src_addrs = [
        SocketAddr::from(([0, 0, 0, 0], 0000)),
        // SocketAddr::from(([0, 0, 0, 0], 9101)),
        // SocketAddr::from(([0, 0, 0, 0], 9102)),
    ];

    let socket = match UdpSocket::bind(&src_addrs[..]) {
        Ok(s) => s,
        Err(e) => return Err(SocketError::BindFailed { cause: e }),
    };

    if let Err(e) = socket.set_broadcast(true) {
        return Err(SocketError::BroadcastFailed { cause: e });
    };

    if let Err(e) = socket.connect(target_ip) {
        return Err(SocketError::ConnectionFailed {
            target_ip: target_ip.to_string(),
            cause: e,
        });
    }

    Ok(socket)
}
