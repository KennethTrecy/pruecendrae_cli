use std::str::{from_utf8, FromStr};
use pruecendrae_core::Request as MaintainerRequest;
use crate::parse::Node;
use crate::spawn_server::request::Request;

const MAX_OUTPUT_SIZE_LABEL: &[u8] = b"max output size";

pub fn parse_output_request<'a>(infos: Vec<Node<'a>>, task_names: Vec<Node<'a>>)
-> Result<Request<'a>, ()> {
	let last_output_size = infos.into_iter().fold(None, |previous_value, info| {
		if let Node::Attacher(label, content) = info {
			if label == MAX_OUTPUT_SIZE_LABEL {
				from_utf8(content)
					.ok()
					.and_then(|new_value| usize::from_str(new_value).ok())
					.and_then(|parsed_value: _| Some(parsed_value))
					.or(previous_value)
			} else {
				previous_value
			}
		} else {
			previous_value
		}
	});

	let max_output_size;
	match last_output_size {
		Some(size) => max_output_size = size,
		None => return Err(())
	}

	let mut names = Vec::with_capacity(task_names.len());
	for name in task_names {
		if let Node::Simplex(name, _) = name {
			let name = from_utf8(name).unwrap();
			names.push(name);
		} else {
			return Err(());
		}
	}

	Ok(Request::Maintainer(MaintainerRequest::Output(max_output_size, names)))
}

#[cfg(test)]
mod t {
	use super::{Node, MaintainerRequest, MAX_OUTPUT_SIZE_LABEL, parse_output_request, Request};

	macro_rules! test {
		($test_name:ident using $infos:expr, and $names:expr, expecting $expected_data:expr) => {
			#[test]
			fn $test_name() {
				let infos = $infos;
				let names = $names;

				let tasks = parse_output_request(infos, names);

				assert_eq!(tasks, $expected_data);
			}
		};
	}

	macro_rules! ok {
		($max_output_size:literal, $task_names:expr) => {
			Ok(
				Request::Maintainer(
					MaintainerRequest::Output($max_output_size, $task_names)
				)
			)
		};
	}

	test!(
		can_parse_output_with_one_name
		using vec![Node::Attacher(MAX_OUTPUT_SIZE_LABEL, b"1")],
		and vec![Node::Simplex(b"task name A", Vec::new())],
		expecting ok!(1, vec!["task name A"])
	);

	test!(
		can_parse_output_with_multiple_names
		using vec![Node::Attacher(MAX_OUTPUT_SIZE_LABEL, b"2")],
		and vec![
			Node::Simplex(b"task name B", Vec::new()),
			Node::Simplex(b"task name C", Vec::new())
		],
		expecting ok!(2, vec!["task name B", "task name C"])
	);

	test!(
		can_parse_output_with_multiple_max_output_size
		using vec![
			Node::Attacher(MAX_OUTPUT_SIZE_LABEL, b"3"),
			Node::Attacher(MAX_OUTPUT_SIZE_LABEL, b"4")
		],
		and vec![Node::Simplex(b"task name D", Vec::new())],
		expecting ok!(4, vec!["task name D"])
	);

	test!(
		can_parse_output_with_incorrect_last_max_output_size_label
		using vec![
			Node::Attacher(MAX_OUTPUT_SIZE_LABEL, b"5"),
			Node::Attacher(b"incorrect max output size label", b"6")
		],
		and vec![Node::Simplex(b"task name E", Vec::new())],
		expecting ok!(5, vec!["task name E"])
	);

	test!(
		can_parse_output_with_incorrect_last_max_output_size_content
		using vec![
			Node::Attacher(MAX_OUTPUT_SIZE_LABEL, b"7"),
			Node::Attacher(MAX_OUTPUT_SIZE_LABEL, b"eight"),
		],
		and vec![Node::Simplex(b"task name F", Vec::new())],
		expecting ok!(7, vec!["task name F"])
	);

	test!(
		cannot_parse_output_with_only_one_but_incorrect_max_output_size_label
		using vec![
			Node::Attacher(b"incorrect max output label", b"9"),
		],
		and vec![Node::Simplex(b"task name F", Vec::new())],
		expecting Err(())
	);

	test!(
		cannot_parse_output_with_only_one_but_incorrect_max_output_size_content
		using vec![
			Node::Attacher(MAX_OUTPUT_SIZE_LABEL, b"ten"),
		],
		and vec![Node::Simplex(b"task name G", Vec::new())],
		expecting Err(())
	);
}
