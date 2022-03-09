use crate::native::{
	UdpSocket,
	FromStr,
	from_utf8
};
use crate::abstracts::{AbstractNode, DynamicAbstractNode};
use crate::concretes::{
	NodeKind,
	Node
};
use crate::log::debug_response;
use crate::migration_utilities::{parse, is_concept_equal};
use crate::{create_local_port, DEFAULT_SERVER_PORT, MAX_BUFFER_SIZE, spawn_server};

pub fn process_configuration_file(configuration: &str) {
	let configuration = configuration.as_bytes();
	let configuration_nodes = parse::<Node, Node>(configuration);

	let mut server_port = DEFAULT_SERVER_PORT;
	let mut initial_state = String::new();

	for node in configuration_nodes {
		if is_concept_equal(configuration, &node, NodeKind::Complex, b"server") {
			let attachers = node.attachers();
			for attacher in attachers {
				if attacher.kind() == NodeKind::Attacher
				&& &configuration[attacher.label().clone()] == b"port" {
					let port = &configuration[attacher.content().clone()];
					let port = from_utf8(port).unwrap();
					server_port = u16::from_str(port)
						.expect("Port in the confguration file is invalid");
				}
			}
		} else if is_concept_equal(configuration, &node, NodeKind::Complex, b"initial state") {
			let raw_commands = node.nodes();
			// Automatically count expected new lines between commands
			let mut total_command_length = raw_commands.len();

			let mut commands = Vec::with_capacity(raw_commands.len());

			for command in raw_commands {
				if command.kind() == NodeKind::BlockOthertongue {
					for line in command.block() {
						total_command_length += line.end - line.start;
						commands.push(&configuration[line.clone()]);
					}
				} else if command.kind() == NodeKind::LineComment
				|| command.kind() == NodeKind::BlockComment {
					continue;
				} else {
					panic!(
						"`initial state` configuration only accepts block othertongues and comments"
					)
				}
			}

			initial_state = String::with_capacity(total_command_length);
			for command in commands {
				initial_state += from_utf8(command).unwrap();
				initial_state += "\n";
			}
		} else {
			panic!("One or more configuration(s) is not supported yet or not at all")
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
