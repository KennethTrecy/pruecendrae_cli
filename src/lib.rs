const MAX_BUFFER_SIZE: usize = 1000;
const DEFAULT_SERVER_PORT: u16 = 7500;

mod log {
	#[cfg(debug_request)]
	use std::str::from_utf8;

	#[cfg(debug_request)]
	pub fn debug_request(requests: &[u8]) {
		println!("Request: {}", from_utf8(requests).unwrap());
	}

	#[cfg(not(debug_request))]
	pub fn debug_request(_: &[u8]) {}
}

mod parse {
	pub use chearmyp::parse::{Node, parse};
}

mod spawn_server;
mod process_task_info;
mod process_configuration_file;

pub use spawn_server::spawn_server;
pub use process_task_info::process_task_info;
pub use process_configuration_file::process_configuration_file;
