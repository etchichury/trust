use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]); //ethernet frame protocol
        if eth_proto != 0x0800 {
            // not IPv4
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(packet) => {
                let src = packet.source_addr();
                let destination = packet.destination_addr();
                let protocol = packet.protocol(); // IP level protocol (expect: TCP)
                eprintln!("{} →  {} | {} bytes of protocol {}", src, destination, packet.payload_len(), protocol);
            },
            Err(e) => {
                eprintln!("Ignoring non IPv4 packet {:?}", e)
            }
        }
    }
}