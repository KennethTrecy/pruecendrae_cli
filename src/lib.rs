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

use pruecendrae_core::TaskMaintainer;
use request::Request;
use parse_requests::parse_requests;

pub fn spawn_server(address: impl ToSocketAddrs) -> JoinHandle<()> {
	let socket = UdpSocket::bind(address).unwrap();
	spawn(move || {
		let mut maintainer = TaskMaintainer::new();
		let mut buffer = [0; MAX_BUFFER_SIZE];

		'server: loop {
			let (size, address) = socket.recv_from(&mut buffer).unwrap();
			if address.ip() != Ipv4Addr::new(127, 0, 0, 1) {
				continue;
			}

			let requests = &buffer[0..size];
			let (requests, are_all_ok) = parse_requests(requests);

			if are_all_ok {
				let mut present_tasks_per_request = Vec::with_capacity(requests.len());
				let mut encoded_responses = String::new();

				for request in requests {
					let request = request.unwrap();
					match request {
						Request::Create(task_infos) => {
							for task_info in task_infos {
								let (task, command) = task_info;
								maintainer.create(task, command).unwrap();
							}
						},
						Request::Maintainer(maintainer_request) => {
							let present_tasks	 = maintainer.send_request(maintainer_request);
							present_tasks_per_request.push(present_tasks);
						},
						Request::List => {
							let task_names = maintainer.list();
							encoded_responses += "list\n\tsuccesses\n";
							for task_name in task_names {
								encoded_responses += &format!("\t\t{}|\n", task_name);
							}
						},
						Request::ForceKill => {
							break 'server;
						}
					}
				}

				for tasks in present_tasks_per_request {
					let response = maintainer.receive_response(tasks);
					encoded_responses += &String::from(response);
				}

				socket.send_to(encoded_responses.as_bytes(), address).unwrap();
			}
		}
	})
}
