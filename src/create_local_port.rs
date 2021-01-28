use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub fn create_local_port(port: u16) -> SocketAddr {
	SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
}
