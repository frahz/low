use low::macaddr::MacAddress;
use low::wol::{create_socket, WolPacket};

const DEFAULT_BROADCAST_IP: &str = "255.255.255.255:9";

fn main() {
    let mac_address = match MacAddress::parse("4C:CC:6A:D0:99:22") {
        Ok(mac_address) => mac_address,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    println!("Mac Address: {:02X?}", mac_address);

    let wol_packet = WolPacket::create(&mac_address);

    let socket = match create_socket(DEFAULT_BROADCAST_IP) {
        Ok(socket) => socket,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    match socket.send_to(&wol_packet.0, DEFAULT_BROADCAST_IP) {
        Ok(_) => println!(
            "Sent packet len: {} {:02X?}",
            &wol_packet.0.len(),
            &wol_packet.0
        ),
        Err(e) => {
            println!("Failed to send packet {}", e);
            std::process::exit(1);
        }
    }
}
