use std::time::Duration;
use std::net::UdpSocket;
use std::thread::{JoinHandle, sleep, spawn};
use pruecendrae_cli::{create_local_port, process_configuration_file};

const WAIT_TIME: Duration = Duration::from_millis(750);

fn create_controller_thread(server_port: u16, controller_port: u16) -> JoinHandle<()> {
	let controller_thread = spawn(move || {
		sleep(WAIT_TIME);
		let socket = UdpSocket::bind(create_local_port(server_port));
		if let Ok(_) = socket {
			panic!("The controller thread had bind to the local socket first.");
		}

		let socket = UdpSocket::bind(create_local_port(controller_port)).unwrap();
		let server_address = create_local_port(server_port);
		socket.send_to(b"force kill|", server_address).unwrap();
	});

	return controller_thread;
}

#[test]
fn can_run_from_configuration_string() {
	let server_local_port = 7510;
	let controller_local_port = 7511;
	let configuration = format!("server\nport: {}", server_local_port);

	let controller = create_controller_thread(server_local_port, controller_local_port);
	process_configuration_file(&configuration);

	assert_eq!(controller.join().unwrap(), ());
}

use std::fs::write;
use std::process::Command;

#[test]
fn can_run_from_configuration_file() {
	let server_local_port = 7512;
	let controller_local_port = 7513;

	let path = "./hidden_tests/can_run_from_configuration.pruecendrae.chearmyp";
	let configuration = format!("server\nport: {}", server_local_port);

	write(path, configuration).unwrap();

	let mut compiled_pruecendrae = Command::new("cargo")
		.args(&["run", "--bin", "main", "--", "--configuration_file", path])
		.spawn()
		.unwrap();

	let controller = create_controller_thread(server_local_port, controller_local_port);

	assert_eq!(controller.join().unwrap(), ());

	sleep(WAIT_TIME);
	assert!(compiled_pruecendrae.try_wait().unwrap().is_some());
}
