use crate::native::{VecDeque, from_utf8};
use crate::abstracts::{
	AbstractAttacherNode,
	AbstractSimplexNode
};
use crate::concretes::Boundary;
use crate::spawn_server::request::Request;

pub fn parse_create_request<'a, T, U>(src: &'a [u8], tasks: &VecDeque<U>)
-> Result<Request<'a>, ()>
where
	T: AbstractAttacherNode<Label = Boundary, Content = Boundary>,
	U: AbstractSimplexNode<Simplex = Boundary, Attachers = VecDeque<T>>
{
	let mut parsed_tasks = Vec::new();

	for task in tasks.into_iter() {
		let name = task.name();
		let infos = task.attachers();
		let mut command = None;
		for info in infos {
			let label = &src[info.label().clone()];
			match label {
				b"command" => command = Some(&src[info.content().clone()]),
				_ => todo!()
			}
		}

		let name = from_utf8(&src[name.clone()]).unwrap();
		let command = command.unwrap();
		let info = (name, command);
		parsed_tasks.push(info);
	}

	Ok(Request::Create(parsed_tasks))
}

#[cfg(test)]
mod t {
	use crate::migration_utilities::parse;
	use crate::concretes::Node;
	use super::{parse_create_request, Request};

	macro_rules! test {
		($test_name:ident using $sample:expr, expecting $expected_data:expr) => {
			#[test]
			fn $test_name() {
				let sample = $sample;

				let tasks = parse_create_request::<Node, Node>(sample, &parse(sample));

				assert_eq!(tasks, $expected_data);
			}
		};
	}

	test!(
		can_parse_one_task
		using b"task name|\n\tcommand: task command",
		expecting Ok(
			Request::Create(
				vec![("task name", b"task command")]
			)
		)
	);

	test!(
		can_parse_multiple_tasks
		using b"task name A|\n\tcommand: task command A\ntask name B|\n\tcommand: task command B",
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
