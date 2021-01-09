use std::thread::sleep;
use std::time::Duration;

fn main() {
	let mut i = 0;
	loop {
		println!("{}", i);
		i += 1;
		sleep(Duration::from_millis(500));
	}
}
