use chearmyp::parse::{Node, parse};
use crate::request::Request;

pub fn parse_requests(src: &[u8]) -> (Vec<Result<Request, ()>>, bool) {
	let nodes = parse(src);
	let mut requests = Vec::with_capacity(nodes.len());

	for node in nodes {
		let request: Result<Request, ()> = match node {
			_ => todo!()
		};
		requests.push(request);
	}

	let are_all_ok = requests.iter().fold(true, |are_ok_previously, request| {
		are_ok_previously && request.is_ok()
	});

	(requests, are_all_ok)
}
