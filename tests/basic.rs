use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;

const TARGET_SEND_SIZE: usize = 10;
const TARGET_PREPARED_RECEIVE_SIZE: usize = 20;
const SEND_DATA: [u8; TARGET_SEND_SIZE] = [1 as u8; TARGET_SEND_SIZE];
const SERVER_ADDRESS: &str = "127.0.0.1:7000";
const CLIENT_ADDRESS: &str = "127.0.0.1:7001";
const READ_TIMEOUT: u64 = 10_000;

#[test]
fn can_receive() {
	let socket = UdpSocket::bind(SERVER_ADDRESS).unwrap();
	let mut data = std::vec![0; TARGET_PREPARED_RECEIVE_SIZE];

	socket.set_read_timeout(Some(Duration::from_millis(READ_TIMEOUT)))
		.expect(&format!("Setting read timeout for address {} failed", SERVER_ADDRESS));

		let (size, address) = socket.recv_from(&mut data).unwrap();

	assert_eq!(size, TARGET_SEND_SIZE);
	assert_eq!(address.to_string(), CLIENT_ADDRESS);
}

#[test]
fn can_send() {
	let socket = UdpSocket::bind(CLIENT_ADDRESS).unwrap();
	sleep(Duration::from_millis(READ_TIMEOUT));
	socket.send_to(&SEND_DATA, SERVER_ADDRESS).unwrap();
}

const RANDOM_SERVER_ADDRESS: &str = "127.0.0.1:7002";
const RANDOM_CLIENT_ADDRESS: &str = "127.0.0.1:0";

#[test]
fn can_receive_from_random_port() {
	let socket = UdpSocket::bind(RANDOM_SERVER_ADDRESS).unwrap();
	let mut data = std::vec![0; TARGET_PREPARED_RECEIVE_SIZE];

	socket.set_read_timeout(Some(Duration::from_millis(READ_TIMEOUT)))
		.expect(&format!("Setting read timeout for address {} failed", RANDOM_SERVER_ADDRESS));

	let (size, address) = socket.recv_from(&mut data).unwrap();

	assert_eq!(size, TARGET_SEND_SIZE);
	println!("Address: {:?}", address);
}

#[test]
fn can_send_from_random_port() {
	let socket = UdpSocket::bind(RANDOM_CLIENT_ADDRESS).unwrap();
	sleep(Duration::from_millis(READ_TIMEOUT));
	socket.send_to(&SEND_DATA, RANDOM_SERVER_ADDRESS).unwrap();
}
