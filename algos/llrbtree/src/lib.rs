use std::cmp::Ord;
enum RB {
	Red,
	Black,
}
use RB::*;
struct RbTreeNode<T:Ord> {
	child: [Box<RbTreeNode<T>>;2],
	value: T,
	color: RB,
}

struct LllrbTree<T:Ord> {
	root: RbTreeNode<T>,
}

impl<T: Ord> RbTreeNode<T> {
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
