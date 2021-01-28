use std::net::UdpSocket;
use super::MAX_BUFFER_SIZE;

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
