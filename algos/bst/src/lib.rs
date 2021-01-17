use std::cmp::{Ord, Ordering};

#[cfg(feature = "bst_derive")]
#[macro_use]
pub mod bst_derive;

pub trait BSTNodeInner: std::marker::Sized {
	fn rotate_left(&mut self);
	fn rotate_right(&mut self);
	// make child in search path root
	#[inline]
	fn rotate_by_ord(&mut self, ord: Ordering) {
		match ord {
			Ordering::Less => self.rotate_right(),
			Ordering::Greater => self.rotate_left(),
			_ => unreachable!(),
		}
	}

	type ChildRaw;
	fn left_mut(&mut self) -> &mut Self::ChildRaw;
	fn right_mut(&mut self) -> &mut Self::ChildRaw;
	fn by_ord_mut(&mut self, ord: Ordering) -> &mut Self::ChildRaw {
		match ord {
			Ordering::Less => self.left_mut(),
			Ordering::Greater => self.right_mut(),
			_ => unreachable!(),
		}
	}
}

pub trait BSTNode: BSTNodeInner {
	type Item: Ord;
	fn left(&mut self) -> Option<Self>;
	fn right(&mut self) -> Option<Self>;

	fn left_ref(&self) -> Option<&Self>;
	fn right_ref(&self) -> Option<&Self>;
	fn by_ord(&self, ord: Ordering) -> Option<&Self> {
		match ord {
			Ordering::Less => self.left_ref(),
			Ordering::Greater => self.right_ref(),
			_ => unreachable!(),
		}
	}

	#[inline]
	fn by_ord_all(&self, ord: Ordering) -> (Option<&Self>, Option<&Self>) {
		match ord {
			Ordering::Less => (self.left_ref(), self.right_ref()),
			Ordering::Greater => (self.right_ref(), self.left_ref()),
			_ => unreachable!(),
		}
	}

	// XXX: maybe return Option<&Self>?
	fn find(&self, k: &Self::Item) -> Option<()> {
		match k.cmp(self.key_ref()) {
			Ordering::Equal => Some(()),
			ord => self.by_ord(ord)?.find(k),
		}
	}

	/*fn left_mut(&mut self) -> Option<&mut Self>;
	fn right_mut(&mut self) -> Option<&mut Self>;*/

	fn key(self) -> Self::Item;
	fn key_ref(&self) -> &Self::Item;
}

//XXX: may split the BSTree into BSTreeBase and BSTreeInner
pub trait BSTree {
	type Item: Ord;
	type Node: BSTNode<Item = Self::Item>;

	fn root_ref(&self) -> Option<&Self::Node>;
	fn root(self) -> Option<Self::Node>;

	fn insert(&mut self, k: Self::Item) -> Option<()>;
	fn remove(&mut self, k: &Self::Item) -> Option<()>;

	fn find(&self, k: &Self::Item) -> Option<()> { self.root_ref()?.find(k) }
}

pub struct Iter<'a, Tree: BSTree> {
	st: Vec<&'a Tree::Node>,
}

impl<'a, Tree: BSTree> Iterator for Iter<'a, Tree>
where <Tree as BSTree>::Item: 'a
{
	type Item = &'a <Tree as BSTree>::Item;

	fn next(&mut self) -> Option<Self::Item> {
		let no = self.st.pop()?;
		let mut r = no.right_ref();
		while let Some(o) = r {
			r = o.left_ref();
			self.st.push(o);
		}
		Some(no.key_ref())
	}
}

impl<'a, Tree: BSTree> Iter<'a, Tree> {
	pub fn from_tree(t: &'a Tree) -> Self {
		let mut st = Vec::new();
		let mut root = t.root_ref();
		while let Some(o) = root {
			root = o.left_ref();
			st.push(o);
		}
		Iter { st }
	}
}

pub struct IntoIter<Tree: BSTree> {
	st: Vec<Tree::Node>,
}

impl<Tree: BSTree> IntoIter<Tree> {
	pub fn from_tree(t: Tree) -> Self {
		let mut st = Vec::new();
		let mut root = t.root();
		while let Some(mut o) = root {
			root = o.left();
			st.push(o)
		}
		IntoIter { st }
	}
}

impl<Tree: BSTree> Iterator for IntoIter<Tree> {
	type Item = <Tree as BSTree>::Item;
	fn next(&mut self) -> Option<Self::Item> {
		let mut no = self.st.pop()?;
		let mut r = no.right();
		while let Some(mut o) = r {
			r = o.left();
			self.st.push(o);
		}

		Some(no.key())
	}
}

//XXX:Is it posible to implement IntoIterator for a type with only trait constraint
// It's not
/*impl<T: Ord, Tree: BSTree<Item=T>> IntoIterator for Tree {
	type Item = T;
	type IntoIter = IntoIter<T, Tree>;
	fn into_iter(self) -> Self::IntoIter {
		let mut st = Vec::new();
		self.root().map(|o| st.push(o));
		IntoIter{ st }
	}
}

impl<'a, T: Ord + 'a, Tree: BSTree<Item=T>> IntoIterator for &'a Tree {
	type Item = &'a T;
	type IntoIter = Iter<'a, T, Tree>;

	fn into_iter(self) -> Self::IntoIter {
		let mut st = Vec::new();
		self.root_ref().map(|o| st.push((o, Ordering::Less)));
		Iter{ st }
	}
}*/
