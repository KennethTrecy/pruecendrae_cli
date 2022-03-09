use pruecendrae_core::Request as MaintainerRequest;
use crate::native::{Vec, VecDeque, from_utf8};
use crate::abstracts::AbstractSimplexNode;
use crate::concretes::Boundary;
use crate::spawn_server::request::Request;

pub fn parse_check_request<'a, T>(src: &'a [u8], task_names: &VecDeque<T>)
-> Result<Request<'a>, ()>
where
	T: AbstractSimplexNode<Simplex = Boundary>
{
	let mut names = Vec::with_capacity(task_names.len());
	for info in task_names {
		let name = from_utf8(&src[info.name().clone()]).unwrap();
		names.push(name);
	}

	Ok(Request::Maintainer(MaintainerRequest::Check(names)))
}

#[cfg(test)]
mod t {
	use crate::migration_utilities::parse;
	use crate::concretes::Node;
	use super::{MaintainerRequest, parse_check_request, Request};

	macro_rules! test {
		($test_name:ident using $names:expr, expecting $expected_data:expr) => {
			#[test]
			fn $test_name() {
				let names = $names;

				let tasks = parse_check_request::<Node>(names, &parse(names));

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
		using b"task name A|",
		expecting ok!["task name A"]
	);

	test!(
		can_parse_check_with_multiple_names
		using b"task name B|\ntask name C|",
		expecting ok!["task name B", "task name C"]
	);
}
