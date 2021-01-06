use std::env::args;
use pruecendrae_cli::spawn_server;

fn main() {
	if args().nth(1).unwrap() == "server" && args().nth(2).unwrap() == "run" {
		spawn_server("127.0.0.1:7500").join().unwrap();
	} else {
		panic!("Currently, the CLI can only support `pruecendrae server run` command.")
	}
}
