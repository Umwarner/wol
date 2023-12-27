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
};

fn main() -> Result<(), ParseIntError> {
    let args: Vec<_> = env::args().collect();

    //TODO: Remove testing code:
    println!("Arguments Passed:");
    for (i, arg) in args.iter().enumerate() {
        println!("Arg {i}: {arg}");
    }

    // mac_address: [u8; 6] Stores mac address in hex/base-16
    // magic_packet - Creates WOL magic packet for the specified mac_address
    let mac_adress = get_mac_address(&args[1])?;
    let magic_packet = wake_on_lan::MagicPacket::new(&mac_adress);

    //TODO: Assumes remote is a hostname, add code to allow hostname or IP to be passed as argument.
    //TODO: Add code to get local GATEWAY-IP
    let local_ip = IpAddr::V4(Ipv4Addr::from([0, 0, 0, 0]));
    let remote_ip = match lookup_host(&args[2]) {
        Ok(ip) => ip[0],
        Err(e) => panic!(
            "Lookup {} failed, do you have the correct hostname? Err: {}",
            args[2], e
        ),
    };

    //Send magic packet to specified host
    match magic_packet.send_to(get_socket(remote_ip, 9), get_socket(local_ip, 9)) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

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

