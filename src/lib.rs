const MAX_BUFFER_SIZE: usize = 1000;
pub const DEFAULT_SERVER_PORT: u16 = 7500;

mod native {
	pub use std::ops::Range;
	pub use std::collections::VecDeque;
	pub use std::vec::Vec;
	pub use std::str::{from_utf8, FromStr};
	pub use std::convert::From;
	pub use std::net::UdpSocket;
	pub use std::cmp::PartialEq;
}

mod log {
	#[cfg(any(feature = "debug_request", feature = "debug_response"))]
	use std::str::from_utf8;

	#[cfg(feature = "debug_request")]
	pub fn debug_request(requests: &[u8]) {
		println!("Request: {}", from_utf8(requests).unwrap());
	}

	#[cfg(not(feature = "debug_request"))]
	pub fn debug_request(_: &[u8]) {}

	#[cfg(feature = "debug_response")]
	pub fn debug_response(requests: &[u8]) {
		println!("Response: {}", from_utf8(requests).unwrap());
	}

	#[cfg(not(feature = "debug_response"))]
	pub fn debug_response(_: &[u8]) {}
}

mod abstracts {
	pub use abstract_chearmyp_node::{
		AbstractNode,
		AbstractNodeQueue,
		AbstractAttacherCollection,
		AbstractAttacherNode,
		AbstractSimplexNode,
		AbstractComplexNode,
		DynamicAbstractNode
	};
}

mod concretes {
	use crate::native::{
		Range,
		Vec,
		VecDeque
	};

	pub use abstract_chearmyp_node::NodeKind;
	use chearmyp_node::Node as GenericNode;
	use chearmyp_token::Token as GenericToken;

	pub type Boundary = Range<usize>;
	pub type BoundaryCollection = Vec<Boundary>;
	pub type Token = GenericToken<Boundary, BoundaryCollection>;
	pub type AttacherToken = Token;
	pub type ScopeLevelToken = Token;
	pub type ComplexToken = Token;
	pub type SimplexToken = Token;
	pub type LineCommentToken = Token;
	pub type BlockCommentToken = Token;
	pub type LineOthertongueToken = Token;
	pub type BlockOthertongueToken = Token;

	pub type Node = GenericNode<Boundary, BoundaryCollection>;
	pub type _NodeForest = VecDeque<Node>;

}

mod chearmyp {
	pub use chearmyp_lexer::lex;
	pub use chearmyp_parser::parse;
}

mod migration_utilities;
mod spawn_server;
mod create_local_port;
mod process_task_info;
mod process_configuration_file;

pub use spawn_server::spawn_server;
pub use create_local_port::create_local_port;
pub use process_task_info::process_task_info;
pub use process_configuration_file::process_configuration_file;
