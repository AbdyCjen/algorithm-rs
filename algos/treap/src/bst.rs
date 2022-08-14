use std::iter::FromIterator;

use crate::*;
use ::bst::derive_bst;

derive_bst! {Treap, TreapNode;
	fn rotate_left(&mut self) {
		let mut right = self.right.take().unwrap();
		self.right = right.left.take();
		std::mem::swap(self, &mut right);
		self.left = Some(right);
	};
	fn rotate_right(&mut self) {
		let mut left = self.left.take().unwrap();
		self.left = left.right.take();
		std::mem::swap(self, &mut left);
		self.right = Some(left);
	}
}

impl<T: Ord> FromIterator<T> for Treap<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut tr = Self::default();
		for v in iter {
			tr.insert(v);
		}
		tr
	}
}

impl<'a, T: Ord + 'a> IntoIterator for &'a Treap<T> {
	type Item = &'a T;
	type IntoIter = ::bst::Iter<'a, Treap<T>>;

	fn into_iter(self) -> Self::IntoIter { ::bst::Iter::from_tree(self) }
}

#[cfg(test)]
mod test {
	use super::*;
	use bst_util::bst_tests::bst_valid;
	#[test]
	fn bst_test() {
		const TEST_RANGE: std::ops::Range<i32> = 0..10_000;
		let mut rbt: Treap<_> = Default::default();
		let test_case: Vec<_> = TEST_RANGE.map(|_| rand::random::<i32>()).collect();
		for &i in &test_case {
			rbt.insert(i);
		}

		bst_valid(&rbt);
		for i in &test_case {
			assert!(rbt.find(i).is_some());
		}
	}
}
