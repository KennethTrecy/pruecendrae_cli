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
