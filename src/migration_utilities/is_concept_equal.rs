use crate::abstracts::{AbstractNode, DynamicAbstractNode};
use crate::concretes::{NodeKind, Node};
use crate::migration_utilities::is_equal_at;

pub fn is_concept_equal(src: &[u8], node: &Node, target_kind: NodeKind, name: &[u8])
-> bool {
	let kind = node.kind();
	kind == target_kind && match kind {
		NodeKind::Simplex | NodeKind::Complex => {
			is_equal_at(src, node.name(), name)
		},
		_ => false
	}
}
