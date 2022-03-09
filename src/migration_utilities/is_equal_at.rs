use crate::concretes::Boundary;

pub fn is_equal_at(src: &[u8], boundary: &Boundary, target: &[u8]) -> bool {
	&src[boundary.clone()] == target
}
