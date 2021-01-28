use std::env::args;
use std::fs::read_to_string;
use pruecendrae_cli::{process_configuration_file, spawn_server};

fn main() {
	let mut configuration = None;
	let mut arguments = args();

	loop {
		let argument = arguments.next();

		match argument {
			Some(argument) => {
				if argument == "--configuration_file" {
					let file_name = arguments.next().expect("Expected name of the file");
					let contents = read_to_string(file_name)
						.expect("Expected file to exist and be readable");
					configuration = Some(contents);
				}
			},
			None => break
		}
	}

	if args().nth(1).unwrap() == "server" && args().nth(2).unwrap() == "run" {
		spawn_server("127.0.0.1:7500").join().unwrap();
	} else if let Some(configuration) = configuration {
		process_configuration_file(&configuration);
	} else {
		panic!("{}\n{}",
			"Currently, the CLI can only support `pruecendrae server run` command.",
			"It also supports `pruecendrae --configuration [filename]` command.")
	}
}
