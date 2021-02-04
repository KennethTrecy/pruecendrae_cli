const MAX_BUFFER_SIZE: usize = 1000;
pub const DEFAULT_SERVER_PORT: u16 = 7500;

mod log {
	#[cfg(any(debug_request, debug_response))]
	use std::str::from_utf8;

	#[cfg(debug_request)]
	pub fn debug_request(requests: &[u8]) {
		println!("Request: {}", from_utf8(requests).unwrap());
	}

	#[cfg(not(debug_request))]
	pub fn debug_request(_: &[u8]) {}

	#[cfg(debug_response)]
	pub fn debug_response(requests: &[u8]) {
		println!("Response: {}", from_utf8(requests).unwrap());
	}

	#[cfg(not(debug_response))]
	pub fn debug_response(_: &[u8]) {}
}

mod parse {
	pub use chearmyp_parser::{Node, parse};
}

mod spawn_server;
mod create_local_port;
mod process_task_info;
mod process_configuration_file;

pub use spawn_server::spawn_server;
pub use create_local_port::create_local_port;
pub use process_task_info::process_task_info;
pub use process_configuration_file::process_configuration_file;
