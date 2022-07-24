use crate::*;
use ::bst;
use std::{cmp::Ord, iter::FromIterator};

impl<T: Ord> IntoIterator for AvlTree<T> {
	type Item = T;
	type IntoIter = bst::IntoIter<AvlTree<T>>;

	fn into_iter(self) -> Self::IntoIter { bst::IntoIter::from_tree(self) }
}

impl<'a, T: Ord + 'a> IntoIterator for &'a AvlTree<T> {
	type Item = &'a T;
	type IntoIter = bst::Iter<'a, AvlTree<T>>;

	fn into_iter(self) -> Self::IntoIter { bst::Iter::from_tree(self) }
}

impl<T: Ord> FromIterator<T> for AvlTree<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut tr: Self = Default::default();
		for v in iter {
			tr.insert(v);
		}
		tr
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_iter() {
		let mut rbt: AvlTree<_> = Default::default();
		const TEST_RANGE: i32 = 1_000_000;
		for i in 0..TEST_RANGE {
			rbt.insert(i);
		}
		assert!((0..TEST_RANGE).eq(rbt.into_iter()));
	}
}
