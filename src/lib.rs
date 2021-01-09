mod request;
mod parse_requests;

use std::net::{UdpSocket, ToSocketAddrs};
use std::thread::{JoinHandle, spawn};

const MAX_BUFFER_SIZE: usize = 1000;

pub fn process_task_info(task: &[u8], address: &str) {
	if let Ok(_socket) = UdpSocket::bind(address) {
		todo!("Create the server")
	} else {
		let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
		socket.connect(address).unwrap();
		socket.send(task).unwrap();
		let mut buffer = [0; MAX_BUFFER_SIZE];
		let _result_size = socket.recv(&mut buffer).unwrap();
		todo!()
	}
}

pub fn spawn_server(address: impl ToSocketAddrs) -> JoinHandle<()> {
	let socket = UdpSocket::bind(address).unwrap();
	spawn(move || {
		let mut buffer = [0; MAX_BUFFER_SIZE];

		loop {
			let (size, _address) = socket.recv_from(&mut buffer).unwrap();
			let _request = &buffer[0..size];
			break;
		}

		todo!();
	})
}

#[cfg(test)]
mod t {
	use super::UdpSocket;

	const TARGET_SEND_SIZE: usize = 10;
	const TARGET_PREPARED_RECEIVE_SIZE: usize = 20;
	const SEND_DATA: [u8; TARGET_SEND_SIZE] = [1 as u8; TARGET_SEND_SIZE];
	const SERVER_ADDRESS: &str = "127.0.0.1:7000";
	const CLIENT_ADDRESS: &str = "127.0.0.1:7001";

	#[test]
	fn can_receive() {
		let socket = UdpSocket::bind(SERVER_ADDRESS).unwrap();
		let mut data = std::vec![0; TARGET_PREPARED_RECEIVE_SIZE];

		let (size, address) = socket.recv_from(&mut data).unwrap();

		assert_eq!(size, TARGET_SEND_SIZE);
		assert_eq!(address.to_string(), CLIENT_ADDRESS);
	}

	#[test]
	fn can_send() {
		let socket = UdpSocket::bind(CLIENT_ADDRESS).unwrap();
		socket.send_to(&SEND_DATA, SERVER_ADDRESS).unwrap();
	}

	const RANDOM_SERVER_ADDRESS: &str = "127.0.0.1:7002";
	const RANDOM_CLIENT_ADDRESS: &str = "127.0.0.1:0";

	#[test]
	fn can_receive_from_random_port() {
		let socket = UdpSocket::bind(RANDOM_SERVER_ADDRESS).unwrap();
		let mut data = std::vec![0; TARGET_PREPARED_RECEIVE_SIZE];

		let (size, address) = socket.recv_from(&mut data).unwrap();

		assert_eq!(size, TARGET_SEND_SIZE);
		println!("Address: {:?}", address);
	}

	#[test]
	fn can_send_from_random_port() {
		let socket = UdpSocket::bind(RANDOM_CLIENT_ADDRESS).unwrap();
		socket.send_to(&SEND_DATA, RANDOM_SERVER_ADDRESS).unwrap();
	}
}
