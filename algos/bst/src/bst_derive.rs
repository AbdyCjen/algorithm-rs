/* for BST declared like
 * ```
 * struct Tree<T> {
 *		root: Option<Box<Node<T>>>
 * }
 * struct Node<T> {
 *		left: Option<Box<Node<T>>>,
 *		right: Option<Box<Node<T>>>,
 *		value: T
 * }
 * ```
 * We can safely use derive_bst to impl basic BST Traits
*/

#[macro_export]
macro_rules! derive_bst {
	($tr_tp: ident, $node_tp: ident; $rotate_left_fn: item; $rotate_right_fn: item) => {
		use ::bst::{BSTNode, BSTNodeInner, BSTree};
		impl<T: std::cmp::Ord> BSTNodeInner for $node_tp<T> {
			type ChildRaw = Option<Box<Self>>;
			#[inline]
			$rotate_left_fn

			#[inline]
			$rotate_right_fn

			#[inline]
			fn left_mut(&mut self) -> &mut Self::ChildRaw { &mut self.left }

			#[inline]
			fn right_mut(&mut self) -> &mut Self::ChildRaw { &mut self.right }
		}

		impl<T: std::cmp::Ord> BSTNode for $node_tp<T> {
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

		impl<T: std::cmp::Ord> BSTree for $tr_tp<T> {
			type Item = T;
			type Node = $node_tp<T>;

			#[inline]
			fn root_ref(&self) -> Option<&Self::Node> { self.root.as_deref() }

			#[inline]
			fn root(self) -> Option<Self::Node> { self.root.map(|o| *o) }

			// FIXME: if remove not manually implemented for $tr_tp, this will cause
			// unconditionally recursion; BSTree::insert too;
			#[inline]
			fn remove(&mut self, k: &Self::Item) -> Option<()> { $tr_tp::remove(self, k) }

			#[inline]
			fn insert(&mut self, k: Self::Item) -> Option<()> { $tr_tp::insert(self, k) }
		}
	}
}
