mod parse_create_request;
mod parse_output_request;
mod parse_check_request;

use crate::abstracts::DynamicAbstractNode;
use crate::concretes::{
	NodeKind,
	Node
};
use crate::migration_utilities::{parse, is_concept_equal};
use crate::spawn_server::request::Request;
use parse_create_request::parse_create_request;
use parse_output_request::parse_output_request;
use parse_check_request::parse_check_request;

pub fn parse_requests(src: &[u8]) -> (Vec<Result<Request, ()>>, bool) {
	let nodes = parse(src);

	let mut requests = Vec::with_capacity(nodes.len());

	for node in nodes.into_iter() {
		let request = if is_concept_equal(src, &node, NodeKind::Complex, b"create") {
			parse_create_request::<Node, Node>(src, node.nodes())
		} else if is_concept_equal(src, &node, NodeKind::Complex, b"output") {
			parse_output_request::<Node, Node>(src, node.attachers(), node.nodes())
		} else if is_concept_equal(src, &node, NodeKind::Complex, b"check") {
			parse_check_request::<Node>(src, node.nodes())
		} else if is_concept_equal(src, &node, NodeKind::Simplex, b"list") {
			Ok(Request::List)
		} else if is_concept_equal(src, &node, NodeKind::Simplex, b"force kill") {
			Ok(Request::ForceKill)
		} else {
			todo!()
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
		let sample = b"create\n\ttask A|\n\t\tcommand: task_a";

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
		let sample = b"output\n\tmax output size: 10\n\ttask B|\n\t\tcommand: task_b";

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
