use crate::native::{VecDeque, FromStr, from_utf8};
use pruecendrae_core::Request as MaintainerRequest;
use crate::abstracts::{
	AbstractAttacherNode,
	AbstractSimplexNode
};
use crate::concretes::Boundary;
use crate::spawn_server::request::Request;

const MAX_OUTPUT_SIZE_LABEL: &[u8] = b"max output size";

pub fn parse_output_request<'a, T, U>(src: &'a [u8], infos: &VecDeque<T>, task_names: &VecDeque<U>)
-> Result<Request<'a>, ()>
where
	T: AbstractAttacherNode<Label = Boundary, Content = Boundary>,
	U: AbstractSimplexNode<Simplex = Boundary, Attachers = VecDeque<T>>
{
	let last_output_size = infos.into_iter().fold(None, |previous_value, info| {
		let label = &src[info.label().clone()];
		if label == MAX_OUTPUT_SIZE_LABEL {
			from_utf8(&src[info.content().clone()])
				.ok()
				.and_then(|new_value| usize::from_str(new_value).ok())
				.and_then(|parsed_value: _| Some(parsed_value))
				.or(previous_value)
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
	for simplex in task_names {
		let name = from_utf8(&src[simplex.name().clone()]).unwrap();
		names.push(name);
	}

	Ok(Request::Maintainer(MaintainerRequest::Output(max_output_size, names)))
}

#[cfg(test)]
mod t {
	use crate::abstracts::AbstractComplexNode;
	use crate::migration_utilities::parse;
	use crate::concretes::Node;
	use super::{
		MaintainerRequest,
		parse_output_request,
		Request
	};

	macro_rules! test {
		($test_name:ident using $sample:expr, expecting $expected_data:expr) => {
			#[test]
			fn $test_name() {
				let sample = parse::<Node, Node>($sample).pop_back().unwrap();
				let infos = sample.attachers();
				let names = sample.nodes();

				let tasks = parse_output_request::<Node, Node>($sample, infos, names);

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
		using b"a\n\tmax output size: 1\n\ttask name A|",
		expecting ok!(1, vec!["task name A"])
	);

	test!(
		can_parse_output_with_multiple_names
		using b"b\n\tmax output size: 2\n\ttask name B|\n\ttask name C|",
		expecting ok!(2, vec!["task name B", "task name C"])
	);

	test!(
		can_parse_output_with_multiple_max_output_size
		using b"c\n\tmax output size: 3\n\tmax output size: 4\n\ttask name D|",
		expecting ok!(4, vec!["task name D"])
	);

	test!(
		can_parse_output_with_incorrect_last_max_output_size_label
		using b"d\n\tmax output size: 5\n\tincorrect max output size label: 6\n\ttask name E|",
		expecting ok!(5, vec!["task name E"])
	);

	test!(
		can_parse_output_with_incorrect_last_max_output_size_content
		using b"e\n\tmax output size: 7\n\tmax output size: eight\n\ttask name F|",
		expecting ok!(7, vec!["task name F"])
	);

	test!(
		cannot_parse_output_with_only_one_but_incorrect_max_output_size_label
		using b"f\n\tincorrect max output label: 9\n\ttask name F|",
		expecting Err(())
	);

	test!(
		cannot_parse_output_with_only_one_but_incorrect_max_output_size_content
		using b"g\n\tmax output size: ten\n\ttask name G|",
		expecting Err(())
	);
}
