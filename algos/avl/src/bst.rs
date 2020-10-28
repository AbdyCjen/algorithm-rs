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
