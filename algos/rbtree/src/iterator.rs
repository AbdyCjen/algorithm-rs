use crate::*;
use ::bst;
use std::cmp::Ord;

impl<T: Ord> IntoIterator for RbTree<T> {
	type Item = T;
	type IntoIter = bst::IntoIter<RbTree<T>>;

	fn into_iter(self) -> Self::IntoIter { bst::IntoIter::from_tree(self) }
}

impl<'a, T: Ord + 'a> IntoIterator for &'a RbTree<T> {
	type Item = &'a T;
	type IntoIter = bst::Iter<'a, RbTree<T>>;

	fn into_iter(self) -> Self::IntoIter { bst::Iter::from_tree(self) }
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_iter() {
		let mut rbt: RbTree<_> = Default::default();
		const TEST_RANGE: i32 = 1_000_000;
		for i in 0..TEST_RANGE {
			rbt.insert(i);
		}
		assert!((0..TEST_RANGE).eq(rbt.into_iter()));
	}
}
