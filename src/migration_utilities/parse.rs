use crate::native::{From, VecDeque};
use crate::abstracts::{AbstractNode, AbstractAttacherNode};
use crate::chearmyp::{lex, parse as parse_syntax};
use crate::concretes::{
	Boundary,
	BoundaryCollection,

	AttacherToken,
	ScopeLevelToken,
	ComplexToken,
	SimplexToken,
	LineCommentToken,
	BlockCommentToken,
	LineOthertongueToken,
	BlockOthertongueToken
};

pub fn parse<T, U>(src: &[u8]) -> VecDeque<U>
where
	T: AbstractAttacherNode<Label = Boundary, Content = Boundary> + From<U>,
	U: AbstractNode<
			usize, Boundary,
			usize, Boundary, BoundaryCollection,
			Boundary, T, VecDeque<T>,
			U, VecDeque<U>
		>
{
	let tokens = lex(&src, VecDeque::new());

	let nodes = parse_syntax::<
		_, _, _, _, _, _, _,
		VecDeque<U>,
		AttacherToken,
		ScopeLevelToken,
		ComplexToken,
		SimplexToken,
		LineCommentToken,
		BlockCommentToken,
		LineOthertongueToken,
		BlockOthertongueToken
	>(tokens);

	return nodes;
}
