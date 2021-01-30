use std::net::UdpSocket;
use std::str::{from_utf8, FromStr};
use crate::log::debug_response;
use crate::parse::{parse, Node};
use crate::{create_local_port, DEFAULT_SERVER_PORT, MAX_BUFFER_SIZE, spawn_server};

pub fn process_configuration_file(configuration: &str) {
	let configuration_nodes = parse(configuration.as_bytes());
	let mut server_port = DEFAULT_SERVER_PORT;
	let initial_state = String::new();

	for node in configuration_nodes {
		match node {
			Node::Complex(b"server", attachers, _) => {
				for attacher in attachers {
					if let Node::Attacher(b"port", port) = attacher {
						let port = from_utf8(port).unwrap();
						server_port = u16::from_str(port)
							.expect("Port in the confguration file is invalid");
					}
				}
			},
			_ => { panic!("One or more configuration(s) is not supported yet or not at all") }
		}
	}

	let server_address = create_local_port(server_port);
	let server = spawn_server(server_address);

	if !initial_state.is_empty() {
		let client_port = create_local_port(0);
		let client_socket = UdpSocket::bind(client_port).unwrap();
		client_socket.send_to(initial_state.as_bytes(), server_address).unwrap();

		let mut response = [0; MAX_BUFFER_SIZE];
		let (size, _) = client_socket.recv_from(&mut response).unwrap();
		let response_content = &response[0..size];
		debug_response(&response_content);
	}

	server.join().unwrap();
}
