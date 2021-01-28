use std::str::from_utf8;
use chearmyp::parse::Node;
use crate::spawn_server::request::Request;

pub fn parse_create_request(tasks: Vec<Node>) -> Result<Request, ()> {
	let mut parsed_tasks = Vec::new();

	for task in tasks {
		if let Node::Simplex(name, infos) = task {
			let mut command = None;
			for info in infos {
				if let Node::Attacher(label, content) = info {
					match label {
						b"command" => command = Some(content),
						_ => todo!()
					}
				}
			}

			let name = from_utf8(name).unwrap();
			let command = command.unwrap();
			let info = (name, command);
			parsed_tasks.push(info);
		} else {
			todo!()
		}
	}

	Ok(Request::Create(parsed_tasks))
}

#[cfg(test)]
mod t {
	use super::{Node, parse_create_request, Request};

	macro_rules! test {
		($test_name:ident using $sample:expr, expecting $expected_data:expr) => {
			#[test]
			fn $test_name() {
				let sample = $sample;

				let tasks = parse_create_request(sample);

				assert_eq!(tasks, $expected_data);
			}
		};
	}

	test!(
		can_parse_one_task
		using vec![
			Node::Simplex(b"task name", vec![
				Node::Attacher(b"command", b"task command")
			])
		],
		expecting Ok(
			Request::Create(
				vec![("task name", b"task command")]
			)
		)
	);

	test!(
		can_parse_multiple_tasks
		using vec![
			Node::Simplex(b"task name A", vec![
				Node::Attacher(b"command", b"task command A")
			]),
			Node::Simplex(b"task name B", vec![
				Node::Attacher(b"command", b"task command B")
			])
		],
		expecting Ok(
			Request::Create(
				vec![
					("task name A", b"task command A"),
					("task name B", b"task command B")
				]
			)
		)
	);
}
