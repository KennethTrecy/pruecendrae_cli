mod parse_create_request;
mod parse_output_request;
mod parse_check_request;

use chearmyp::parse::{Node, parse};
use crate::spawn_server::request::Request;
use parse_create_request::parse_create_request;
use parse_output_request::parse_output_request;
use parse_check_request::parse_check_request;

pub fn parse_requests(src: &[u8]) -> (Vec<Result<Request, ()>>, bool) {
	let nodes = parse(src);
	let mut requests = Vec::with_capacity(nodes.len());

	for node in nodes {
		let request = match node {
			Node::Complex(b"create", _, tasks) => parse_create_request(tasks),
			Node::Complex(b"output", infos, task_names) => parse_output_request(infos, task_names),
			Node::Complex(b"check", _, task_names) => parse_check_request(task_names),
			Node::Simplex(b"list", _) => Ok(Request::List),
			Node::Simplex(b"force kill", _) => Ok(Request::ForceKill),
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

	use pruecendrae_core::Request as MaintainerRequest;

	#[test]
	pub fn can_parse_output_request() {
		let sample = b"output\nmax output size: 10\n\ttask B|\n\tcommand: task_b";

		let parsed_requests = parse_requests(sample);

		assert_eq!(parsed_requests, (vec![
			Ok(
				Request::Maintainer(MaintainerRequest::Output(10, vec!["task B"]))
			)
		], true));
	}

	#[test]
	pub fn can_parse_list_request() {
		let sample = b"list|\n";

		let parsed_requests = parse_requests(sample);

		assert_eq!(parsed_requests, (vec![Ok(Request::List)], true));
	}
}
