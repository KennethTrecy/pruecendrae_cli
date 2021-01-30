use std::str::{from_utf8, FromStr};
use crate::parse::{parse, Node};
use crate::{create_local_port, DEFAULT_SERVER_PORT, spawn_server};

pub fn process_configuration_file(configuration: &str) {
	let configuration_nodes = parse(configuration.as_bytes());
	let mut server_port = DEFAULT_SERVER_PORT;

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

	server.join().unwrap();
}
