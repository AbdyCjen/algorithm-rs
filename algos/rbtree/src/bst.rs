use crate::*;
use ::bst::{BSTNode, BSTNodeInner, BSTree};

impl<T: Ord> BSTNodeInner for RbTreeNode<T> {
	type ChildRaw = Option<Box<Self>>;
	#[inline]
	fn rotate_left(&mut self) {
		let mut right = self.right.take().unwrap();
		self.right = right.left.take();
		right.color = self.color;
		self.color = Red;
		std::mem::swap(self, &mut right);
		self.left = Some(right);
	}

	#[inline]
	fn rotate_right(&mut self) {
		let mut left = self.left.take().unwrap();
		self.left = left.right.take();
		left.color = self.color;
		self.color = Red;
		std::mem::swap(self, &mut left);
		self.right = Some(left);
	}

	#[inline]
	fn left_mut(&mut self) -> &mut Self::ChildRaw { &mut self.left }

	#[inline]
	fn right_mut(&mut self) -> &mut Self::ChildRaw { &mut self.right }
}

impl<T: Ord> BSTNode for RbTreeNode<T> {
	type Item = T;

	#[inline]
	fn left(&mut self) -> Option<Self> { self.left.take().map(|o| *o) }

	#[inline]
	fn right(&mut self) -> Option<Self> { self.right.take().map(|o| *o) }

	#[inline]
	fn left_ref(&self) -> Option<&Self> { self.left.as_deref() }

	#[inline]
	fn right_ref(&self) -> Option<&Self> { self.right.as_deref() }

	#[inline]
	fn key(self) -> Self::Item { self.k }

	#[inline]
	fn key_ref(&self) -> &Self::Item { &self.k }
}

impl<T: Ord> BSTree for RBTree<T> {
	type Item = T;
	type Node = RbTreeNode<T>;

	#[inline]
	fn root_ref(&self) -> Option<&Self::Node> { self.root.as_ref() }

	#[inline]
	fn root(self) -> Option<Self::Node> { self.root }

	#[inline]
	fn remove(&mut self, k: &Self::Item) -> Option<()> { self.remove(k) }

	#[inline]
	fn insert(&mut self, k: Self::Item) -> Option<()> { self.insert(k) }
}

#[cfg(test)]
mod test {
	use super::*;
	use bst_util::bst_tests::bst_valid;
	const TEST_RANGE: std::ops::Range<i32> = 0..1_000_000;
	#[test]
	fn bst_test() {
		let mut rbt: RBTree<_> = Default::default();
		let mut test_case = Vec::new();
		for _ in TEST_RANGE {
			test_case.push(rand::random::<i32>());
		}
		for i in test_case.iter().copied() {
			rbt.insert(i);
		}

		bst_valid(&rbt);
		for i in &test_case {
			assert!(rbt.find(i).is_some());
		}
	}
}
