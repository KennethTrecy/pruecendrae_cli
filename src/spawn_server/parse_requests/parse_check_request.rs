use std::str::from_utf8;
use chearmyp::parse::Node;
use pruecendrae_core::Request as MaintainerRequest;
use crate::spawn_server::request::Request;

pub fn parse_check_request(task_names: Vec<Node>) -> Result<Request, ()> {
	let mut names = Vec::with_capacity(task_names.len());
	for name in task_names {
		if let Node::Simplex(name, _) = name {
			let name = from_utf8(name).unwrap();
			names.push(name);
		} else {
			return Err(());
		}
	}

	Ok(Request::Maintainer(MaintainerRequest::Check(names)))
}

#[cfg(test)]
mod t {
	use super::{Node, MaintainerRequest, parse_check_request, Request};

	macro_rules! test {
		($test_name:ident using $names:expr, expecting $expected_data:expr) => {
			#[test]
			fn $test_name() {
				let names = $names;

				let tasks = parse_check_request(names);

				assert_eq!(tasks, $expected_data);
			}
		};
	}

	macro_rules! ok {
		($($task_names:expr),+) => {
			Ok(
				Request::Maintainer(
					MaintainerRequest::Check(vec![$($task_names,)+])
				)
			)
		};
	}

	test!(
		can_parse_check_with_one_name
		using vec![Node::Simplex(b"task name A", Vec::new())],
		expecting ok!["task name A"]
	);

	test!(
		can_parse_check_with_multiple_names
		using vec![
			Node::Simplex(b"task name B", Vec::new()),
			Node::Simplex(b"task name C", Vec::new())
		],
		expecting ok!["task name B", "task name C"]
	);
}
