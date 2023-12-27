// Takes mac address and remote gateway IP and sends a magic packet to wake the target device
//
// Usage:
// wol.exe MAC-ADDRESS GATEWAY-IP

use core::panic;
use dns_lookup::lookup_host;
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::ParseIntError,
    str::FromStr,
};

fn main() -> Result<(), ParseIntError> {
    let args: Vec<_> = env::args().collect();

    // mac_address: [u8; 6] Stores mac address in hex/base-16
    // magic_packet - Creates WOL magic packet for the specified mac_address
    // local_ip = Doesn't appear to be neccessary??? using 0.0.0.0
    // remote_ip = IP of remote gateway, can be IP or DNS hostname
    let mac_address = get_mac_address(&args[1])?;
    let magic_packet = wake_on_lan::MagicPacket::new(&mac_address);
    let local_ip = IpAddr::V4(Ipv4Addr::from([0, 0, 0, 0]));
    let remote_ip = get_remote_ip(&args[2]);

    //Send magic packet to specified host on port 9
    // TODO: Allow user to specify port
    match magic_packet.send_to(get_socket(remote_ip, 9), get_socket(local_ip, 9)) {
        Ok(_) => (),
        Err(e) => panic!("Magic Packet Failed to Send: {}", e),
    }

    println!("Magic Packet sent to:");
    println!("IP Address: {}", remote_ip);
    println!("Mac Address: {:?}", mac_address);
    Ok(())
}

fn get_mac_address(arg: &str) -> Result<[u8; 6], ParseIntError> {
    let mut mac_adress: [u8; 6] = [0; 6];
    for (i, segment) in arg.split(&[':', '-']).enumerate() {
        mac_adress[i] = u8::from_str_radix(segment, 16)?;
    }
    Ok(mac_adress)
}

fn get_socket(ip: IpAddr, port: u16) -> SocketAddr {
    match ip {
        IpAddr::V4(ipv4) => SocketAddr::from((ipv4, port)),
        IpAddr::V6(ipv6) => SocketAddr::from((ipv6, port)),
    }
}

// Checks if IP is valid, if not attempts to resolve as DNS hostname. If both fail, panic!
fn get_remote_ip(address: &str) -> IpAddr {
    match IpAddr::from_str(address) {
        Ok(ip) => ip,
        Err(_) => match lookup_host(address) {
            Ok(ip) => ip[0],
            Err(e) => panic!("Host {} does not appear to be valid. Error: {}", address, e),
        },
    }
}

