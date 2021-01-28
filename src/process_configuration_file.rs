use chearmyp::parse::parse;

pub fn process_configuration_file(configuration: &str) {
	let _parsed_configuration = parse(configuration.as_bytes());
}
