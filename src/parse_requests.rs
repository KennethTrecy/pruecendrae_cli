use chearmyp::parse::{Node, parse};
use crate::request::Request;

mod parse_create_request;

use parse_create_request::parse_create_request;

pub fn parse_requests(src: &[u8]) -> (Vec<Result<Request, ()>>, bool) {
	let nodes = parse(src);
	let mut requests = Vec::with_capacity(nodes.len());

	for node in nodes {
		let request = match node {
			Node::Complex(b"create", _, tasks) => parse_create_request(tasks),
			_ => todo!()
		};
		requests.push(request);
	}

	let are_all_ok = requests.iter().fold(true, |are_ok_previously, request| {
		are_ok_previously && request.is_ok()
	});

	(requests, are_all_ok)
}

#[cfg(test)]
mod t {
	use super::{parse_requests, Request};

	#[test]
	pub fn can_parse_create_request() {
		let sample = b"create\n\ttask A|\n\tcommand: task_a";

		let parsed_requests = parse_requests(sample);

		assert_eq!(parsed_requests, (vec![
			Ok(
				Request::Create(vec![("task A", b"task_a")])
			)
		], true));
	}
}
