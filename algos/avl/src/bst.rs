use crate::*;
use ::bst::{BSTNode, BSTNodeInner, BSTree};

impl<T: Ord> BSTNodeInner for AvlNode<T> {
	type ChildRaw = Option<Box<Self>>;

	#[inline]
	fn rotate_left(&mut self) {
		// FIXME: height
		let mut right = self.right.take().unwrap();
		self.right = right.left.take();
		self.update_height();
		std::mem::swap(self, &mut right);
		self.left = Some(right);
		self.update_height();
	}
	#[inline]
	fn rotate_right(&mut self) {
		// FIXME: height
		let mut left = self.left.take().unwrap();
		self.left = left.right.take();
		self.update_height();
		std::mem::swap(self, &mut left);
		self.right = Some(left);
		self.update_height();
	}

	#[inline]
	fn left_mut(&mut self) -> &mut Self::ChildRaw { &mut self.left }

	#[inline]
	fn right_mut(&mut self) -> &mut Self::ChildRaw { &mut self.right }
}

impl<T: Ord> BSTNode for AvlNode<T> {
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

impl<T: Ord> BSTree for AvlTree<T> {
	type Item = T;
	type Node = AvlNode<T>;

	#[inline]
	fn root_ref(&self) -> Option<&Self::Node> { self.root.as_deref() }

	#[inline]
	fn root(self) -> Option<Self::Node> { self.root.map(|o| *o) }

	#[inline]
	fn remove(&mut self, k: &Self::Item) -> Option<()> { self.remove(k) }

	#[inline]
	fn insert(&mut self, k: Self::Item) -> Option<()> { self.insert(k) }
}

#[cfg(test)]
mod test{
	use super::*;
	use bst_util::bst_tests::*;
	#[test]
	fn bst_basic() {
		let mut tr : AvlTree<_> = Default::default();
		let mut test_case = Vec::new();
		const TEST_RANGE: std::ops::Range<i32> = 0..1_000_000;
		for _ in TEST_RANGE {
			test_case.push(rand::random::<i32>());
		}
		
		for i in test_case.iter().copied() {
			tr.insert(i);
		}
		bst_valid(&tr);

		for i in &test_case {
			assert!(tr.find(i).is_some());
		}
	}
}
