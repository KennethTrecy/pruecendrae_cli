use std::fs::write;
use std::net::UdpSocket;
use std::time::Duration;
use std::process::Command;
use std::thread::{JoinHandle, sleep, spawn};
use pruecendrae_cli::{create_local_port, process_configuration_file};

macro_rules! test {
	(
		using server port $string_server_port:literal and control port $string_control_port:literal,
		$string_test_name:ident expecting $expected_output_from_string_test:expr;

		using server port $file_server_port:literal and control port $file_control_port:literal,
		$file_test_name:ident expecting $expected_output_from_file_test:expr;
	) => {
		#[test]
		fn $string_test_name() {
			let server_port = $string_server_port;
			let controller_port = $string_control_port;
			let configuration = format!("server\nport: {}", server_port);

			let controller = create_controller_thread(server_port, controller_port);
			process_configuration_file(&configuration);

			assert_eq!(controller.join().unwrap(), $expected_output_from_string_test);
		}

		#[test]
		fn $file_test_name() {
			let server_port = $file_server_port;
			let controller_port = $file_control_port;

			let path = &format!("./hidden_tests/{}.pruecendrae.chearmyp", stringify!($file_test_name));
			let configuration = format!("server\nport: {}", server_port);

			write(path, configuration).unwrap();

			let mut compiled_pruecendrae = Command::new("cargo")
				.args(&["run", "--bin", "main", "--", "--configuration_file", path])
				.spawn()
				.unwrap();

			let controller = create_controller_thread(server_port, controller_port);

			assert_eq!(controller.join().unwrap(), $expected_output_from_file_test);

			sleep(WAIT_TIME);
			assert!(compiled_pruecendrae.try_wait().unwrap().is_some());
		}
	};
}

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

test!{
	using server port 7510 and control port 7511,
	can_run_from_configuration_string expecting ();

	using server port 7512 and control port 7513,
	can_run_from_configuration_file expecting ();
}
