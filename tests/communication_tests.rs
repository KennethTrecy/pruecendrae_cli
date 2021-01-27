use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::Duration;
use pruecendrae_cli::spawn_server;

macro_rules! test {
	(
		using port $port:literal,
		$test_name:ident with $payload:literal
		$(then manipulate server by $function:ident)?
		expecting $expected_reply:literal
	) => {
		#[test]
		fn $test_name() {
			let server_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), $port);
			let _server = spawn_server(server_address);
			let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
			socket.set_read_timeout(Some(Duration::from_millis(500)))
				.expect(&format!("Setting read timeout for port {} failed", $port));

			socket.send_to(&$payload[..], &server_address).unwrap();
			let mut buffer = [0; 1000];
			let (size, address) = socket.recv_from(&mut buffer).unwrap_or_else(|_| (0, server_address));

			$(
				let server = _server;
				$function(server);
			)?

			assert_eq!(address, server_address);
			assert_eq!(&buffer[0..size], $expected_reply);
		}
	};
}

test!{
	using port 7501, can_create_task
	with b"create\n\tfake|\n\tcommand: cargo run --bin fake_program"
	expecting b""
}

test!{
	using port 7502, can_output_task
	with b"create\n\tfake|\n\tcommand: cargo run --bin fake_program\noutput\nmax output size: 100\n\tfake|\n"
	expecting b"output\n\tsuccesses\n\t\tfake\n\t\t\t===\n0\n\n\t\t\t===\n"
}

test!{
	using port 7503, can_check_task
	with b"create\n\tfake|\n\tcommand: cargo run --bin fake_program\ncheck\n\tfake|\n"
	expecting b"check\n\tsuccesses\n\t\tfake|\n"
}

use std::thread::JoinHandle;

fn join_server(server: JoinHandle<()>) {
	server.join().unwrap();
}

test!{
	using port 7504, can_force_kill_server
	with b"create\n\tfake|\n\tcommand: cargo run --bin fake_program\nforce kill|\n"
	then manipulate server by join_server
	expecting b""
}

test!{
	using port 7505, can_list_task
	with b"create\n\tfake|\n\tcommand: cargo run --bin fake_program\nlist|\n"
	expecting b"list\n\tsuccesses\n\t\tfake|\n"
}
