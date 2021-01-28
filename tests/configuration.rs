use std::time::Duration;
use std::net::UdpSocket;
use std::thread::{sleep, spawn};
use pruecendrae_cli::{create_local_port, process_configuration_file};

#[test]
fn can_run_from_configuration() {
	let server_local_port = 7510;
	let controller_local_port = 7511;
	let configuration = format!("server\nport: {}", server_local_port);

	let controller_thread = spawn(move || {
		sleep(Duration::from_millis(5000));
		let socket = UdpSocket::bind(create_local_port(server_local_port));
		if let Ok(_) = socket {
			panic!("The controller thread had bind to the local socket first.");
		}

		let socket = UdpSocket::bind(create_local_port(controller_local_port)).unwrap();
		let server_address = create_local_port(server_local_port);
		socket.send_to(b"force kill|", server_address).unwrap();
	});
	process_configuration_file(&configuration);

	assert_eq!(controller_thread.join().unwrap(), ());
}
