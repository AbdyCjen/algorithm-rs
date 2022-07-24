use arrayvec::ArrayVec;
use std::cmp::Ord;
pub mod iter;

const B: usize = 6;
const MAX_CHILD: usize = B * 2;
const CAPACITY: usize = 2 * B - 1;
const MIN_LEN_AFTER_SPLIT: usize = B - 1;

pub enum BTree<T> {
	Leaf(BTreeLeafNode<T>),
	Inner(BTreeInnerNode<T>),
}

impl<T> Default for BTree<T> {
	fn default() -> Self {
		Self::Leaf(BTreeLeafNode {
			keys: ArrayVec::new(),
		})
	}
}

pub(crate) enum Children<T> {
	Leaf(ArrayVec<Box<BTreeLeafNode<T>>, MAX_CHILD>),
	Inner(ArrayVec<Box<BTreeInnerNode<T>>, MAX_CHILD>),
}

impl<T> Default for Children<T> {
	fn default() -> Self { Self::Leaf(ArrayVec::new()) }
}

impl<T> From<ArrayVec<Box<BTreeInnerNode<T>>, MAX_CHILD>> for Children<T> {
	fn from(arr: ArrayVec<Box<BTreeInnerNode<T>>, MAX_CHILD>) -> Self { Self::Inner(arr) }
}

impl<T> From<ArrayVec<Box<BTreeLeafNode<T>>, MAX_CHILD>> for Children<T> {
	fn from(arr: ArrayVec<Box<BTreeLeafNode<T>>, MAX_CHILD>) -> Self { Self::Leaf(arr) }
}

#[derive(Default)]
pub struct BTreeInnerNode<T> {
	keys: ArrayVec<T, CAPACITY>,
	children: Children<T>,
}

#[derive(Default)]
pub struct BTreeLeafNode<T> {
	keys: ArrayVec<T, CAPACITY>,
}

impl<T: Ord> BTree<T> {
	pub fn insert(&mut self, key: T) {
		match self {
			Self::Leaf(root) => {
				if let Some((new_key, new_tr)) = root.insert(key) {
					unsafe {
						let orig_tr = Box::new(std::ptr::read(root as _));
						std::ptr::write(
							self as _,
							Self::Inner(BTreeInnerNode {
								keys: std::iter::once(new_key).collect(),
								children: ArrayVec::from_iter([orig_tr, new_tr]).into(),
							}),
						);
					}
				}
			}
			Self::Inner(root) => {
				if let Some((new_key, new_tr)) = root.insert(key) {
					unsafe {
						let orig_tr = Box::new(std::ptr::read(root as _));
						std::ptr::write(
							self as _,
							Self::Inner(BTreeInnerNode {
								keys: std::iter::once(new_key).collect(),
								children: ArrayVec::from_iter([orig_tr, new_tr]).into(),
							}),
						);
					}
				}
			}
		}
	}

	pub fn remove(&mut self, key: &T) -> Option<T> {
		match self {
			Self::Leaf(root) => root.remove(key),
			Self::Inner(root) => {
				let to_ret = root.remove(key);
				if root.keys.is_empty() {
					*self = match &mut root.children {
						Children::Leaf(child) => Self::Leaf(*child.pop().unwrap()),
						Children::Inner(child) => Self::Inner(*child.pop().unwrap()),
					}
				}
				to_ret
			}
		}
	}

	pub fn find(&self, key: &T) -> bool {
		match self {
			Self::Leaf(root) => root.find(key).is_some(),
			Self::Inner(root) => root.find(key).is_some(),
		}
	}
}

impl<T: Ord> BTreeInnerNode<T> {
	fn find(&self, key: &T) -> Option<()> {
		match self.keys.binary_search(key) {
			Ok(_) => Some(()),
			Err(pos) => match &self.children {
				Children::Inner(children) => children[pos].find(key),
				Children::Leaf(children) => children[pos].find(key),
			},
		}
	}

	fn insert(&mut self, key: T) -> Option<(T, Box<BTreeInnerNode<T>>)> {
		let pos = self.keys.binary_search(&key).err()?;

		match &mut self.children {
			Children::Inner(children) => {
				if self.keys.is_full() {
					let mut split_childs: ArrayVec<Box<Self>, MAX_CHILD> =
						children.drain((MIN_LEN_AFTER_SPLIT + 1)..).collect();
					let mut split_keys: ArrayVec<T, CAPACITY> =
						self.keys.drain((MIN_LEN_AFTER_SPLIT + 1)..).collect();
					let split_key = self.keys.pop().unwrap();

					if pos <= MIN_LEN_AFTER_SPLIT {
						if let Some((new_key, new_tr)) = children[pos].insert(key) {
							self.keys.insert(pos, new_key);
							children.insert(pos, new_tr);
						}
					} else if let Some((new_key, new_tr)) =
						split_childs[pos - MIN_LEN_AFTER_SPLIT - 1].insert(key)
					{
						split_keys.insert(pos - MIN_LEN_AFTER_SPLIT - 1, new_key);
						split_childs.insert(pos - MIN_LEN_AFTER_SPLIT, new_tr);
					}

					Some((
						split_key,
						Box::new(Self {
							keys: split_keys,
							children: split_childs.into(),
						}),
					))
				} else {
					if let Some((new_key, new_tr)) = children[pos].insert(key) {
						self.keys.insert(pos, new_key);
						children.insert(pos + 1, new_tr);
					}
					None
				}
			}
			Children::Leaf(children) => {
				if self.keys.is_full() {
					let mut split_childs: ArrayVec<Box<BTreeLeafNode<T>>, MAX_CHILD> =
						children.drain((MIN_LEN_AFTER_SPLIT + 1)..).collect();
					let mut split_keys: ArrayVec<T, CAPACITY> =
						self.keys.drain((MIN_LEN_AFTER_SPLIT + 1)..).collect();
					let split_key = self.keys.pop().unwrap();

					if pos <= MIN_LEN_AFTER_SPLIT {
						if let Some((new_key, new_tr)) = children[pos].insert(key) {
							self.keys.insert(pos, new_key);
							children.insert(pos, new_tr);
						}
					} else if let Some((new_key, new_tr)) =
						split_childs[pos - MIN_LEN_AFTER_SPLIT - 1].insert(key)
					{
						split_keys.insert(pos - MIN_LEN_AFTER_SPLIT - 1, new_key);
						split_childs.insert(pos - MIN_LEN_AFTER_SPLIT, new_tr);
					}

					Some((
						split_key,
						Box::new(Self {
							keys: split_keys,
							children: split_childs.into(),
						}),
					))
				} else {
					if let Some((new_key, new_tr)) = children[pos].insert(key) {
						self.keys.insert(pos, new_key);
						children.insert(pos + 1, new_tr);
					}
					None
				}
			}
		}
	}

	fn remove(&mut self, key: &T) -> Option<T> {
		match self.keys.binary_search(key) {
			Ok(pos) => match &mut self.children {
				Children::Leaf(children) => {
					let key = if children[pos].keys.len() > MIN_LEN_AFTER_SPLIT {
						std::mem::replace(&mut self.keys[pos], children[pos].remove_rightmost())
					} else if children[pos + 1].keys.len() > MIN_LEN_AFTER_SPLIT {
						std::mem::replace(&mut self.keys[pos], children[pos + 1].remove_leftmost())
					} else {
						let new_key = self.keys.remove(pos);
						let right = children.remove(pos + 1);
						children[pos].keys.push(new_key);
						children[pos].keys.extend(right.keys);
						return children[pos].remove(key);
					};
					Some(key)
				}
				Children::Inner(children) => {
					let key = if children[pos].keys.len() > MIN_LEN_AFTER_SPLIT {
						std::mem::replace(&mut self.keys[pos], children[pos].remove_rightmost())
					} else if children[pos + 1].keys.len() > MIN_LEN_AFTER_SPLIT {
						std::mem::replace(&mut self.keys[pos], children[pos + 1].remove_leftmost())
					} else {
						let new_key = self.keys.remove(pos);
						let right = children.remove(pos + 1);
						let child = &mut children[pos];
						child.keys.push(new_key);
						child.keys.extend(right.keys);
						match (&mut child.children, right.children) {
							(Children::Leaf(lchilds), Children::Leaf(rchilds)) => {
								lchilds.extend(rchilds)
							}
							(Children::Inner(lchilds), Children::Inner(rchilds)) => {
								lchilds.extend(rchilds)
							}
							_ => unreachable!(),
						}
						return child.remove(key);
					};
					Some(key)
				}
			},
			Err(pos) => match &mut self.children {
				Children::Leaf(children) => {
					if children[pos].keys.len() > MIN_LEN_AFTER_SPLIT {
						children[pos].remove(key)
					} else if pos + 1 < children.len()
						&& children[pos + 1].keys.len() > MIN_LEN_AFTER_SPLIT
					{
						let new_key = std::mem::replace(
							self.keys.get_mut(pos).unwrap(),
							children[pos + 1].keys.remove(0),
						);
						children[pos].keys.push(new_key);
						children[pos].remove(key)
					} else if pos > 0 && children[pos - 1].keys.len() > MIN_LEN_AFTER_SPLIT {
						let new_key = std::mem::replace(
							self.keys.get_mut(pos - 1).unwrap(),
							children[pos - 1].keys.pop().unwrap(),
						);
						children[pos].keys.insert(0, new_key);
						children[pos].remove(key)
					} else {
						let pivot = pos.saturating_sub(1);
						let right = children.remove(pivot + 1);
						let child = children.get_mut(pivot).unwrap();
						child.keys.push(self.keys.remove(pivot));
						child.keys.extend(right.keys);
						child.remove(key)
					}
				}
				Children::Inner(children) => {
					if children[pos].keys.len() > MIN_LEN_AFTER_SPLIT {
						children[pos].remove(key)
					} else if pos + 1 < children.len()
						&& children[pos + 1].keys.len() > MIN_LEN_AFTER_SPLIT
					{
						let new_key = std::mem::replace(
							self.keys.get_mut(pos).unwrap(),
							children[pos + 1].keys.remove(0),
						);
						children[pos].keys.push(new_key);
						match &mut children[pos + 1].children {
							Children::Leaf(rchildren) => {
								let split_tr = rchildren.remove(0);
								if let Children::Leaf(lchildren) = &mut children[pos].children {
									lchildren.push(split_tr);
								}
							}
							Children::Inner(rchildren) => {
								let split_tr = rchildren.remove(0);
								if let Children::Inner(lchildren) = &mut children[pos].children {
									lchildren.push(split_tr);
								}
							}
						}
						children[pos].remove(key)
					} else if pos > 0 && children[pos - 1].keys.len() > MIN_LEN_AFTER_SPLIT {
						let new_key = std::mem::replace(
							self.keys.get_mut(pos - 1).unwrap(),
							children[pos - 1].keys.pop().unwrap(),
						);
						children[pos].keys.insert(0, new_key);
						match &mut children[pos - 1].children {
							Children::Leaf(lchildren) => {
								let split_tr = lchildren.pop().unwrap();
								if let Children::Leaf(rchildren) = &mut children[pos].children {
									rchildren.insert(0, split_tr);
								}
							}
							Children::Inner(lchildren) => {
								let split_tr = lchildren.pop().unwrap();
								if let Children::Inner(rchildren) = &mut children[pos].children {
									rchildren.insert(0, split_tr);
								}
							}
						}
						children[pos].remove(key)
					} else {
						//merge
						let pivot = pos.saturating_sub(1);
						let right = children.remove(pivot + 1);
						let child = children.get_mut(pivot).unwrap();
						child.keys.push(self.keys.remove(pivot));
						child.keys.extend(right.keys);
						match (&mut child.children, right.children) {
							(Children::Leaf(lchildren), Children::Leaf(rchildren)) => {
								lchildren.extend(rchildren);
							}
							(Children::Inner(lchildren), Children::Inner(rchildren)) => {
								lchildren.extend(rchildren)
							}
							_ => unreachable!(),
						}
						child.remove(key)
					}
				}
			},
		}
	}

	fn remove_rightmost(&mut self) -> T {
		let pos = self.keys.len() - 1;
		match &mut self.children {
			Children::Leaf(children) => {
				let rightmost_child = children.last_mut().unwrap();
				if rightmost_child.keys.len() > MIN_LEN_AFTER_SPLIT {
					rightmost_child.remove_rightmost()
				} else if children[pos].keys.len() > MIN_LEN_AFTER_SPLIT {
					// split 1 key to rightmost tree
					let new_key = std::mem::replace(
						self.keys.last_mut().unwrap(),
						children[pos].keys.pop().unwrap(),
					);
					let rightmost_child = children.last_mut().unwrap();
					rightmost_child.keys.insert(0, new_key);
					rightmost_child.remove_rightmost()
				} else {
					// merge the last two children
					let new_key = self.keys.pop().unwrap();
					let right = children.pop().unwrap();
					let rightmost_child = children.last_mut().unwrap();
					rightmost_child.keys.push(new_key);
					rightmost_child.keys.extend(right.keys);
					rightmost_child.remove_rightmost()
				}
			}
			Children::Inner(children) => {
				let rightmost_child = children.last_mut().unwrap();
				if rightmost_child.keys.len() > MIN_LEN_AFTER_SPLIT {
					rightmost_child.remove_rightmost()
				} else if children[pos].keys.len() > MIN_LEN_AFTER_SPLIT {
					// split 1 key to rightmost tree
					let split_key = std::mem::replace(
						self.keys.last_mut().unwrap(),
						children[pos].keys.pop().unwrap(),
					);
					let mut right = children.pop().unwrap();
					let rightmost_child = children.last_mut().unwrap();
					match (&mut rightmost_child.children, &mut right.children) {
						(Children::Leaf(lchild), Children::Leaf(rchild)) => {
							rchild.insert(0, lchild.pop().unwrap());
						}
						(Children::Inner(lchild), Children::Inner(rchild)) => {
							rchild.insert(0, lchild.pop().unwrap());
						}
						_ => unreachable!(),
					}
					right.keys.insert(0, split_key);
					children.push(right);
					children.last_mut().unwrap().remove_rightmost()
				} else {
					let new_key = self.keys.pop().unwrap();
					let right = children.pop().unwrap();
					let rightmost_child = children.last_mut().unwrap();
					rightmost_child.keys.push(new_key);
					rightmost_child.keys.extend(right.keys);
					match (&mut rightmost_child.children, right.children) {
						(Children::Leaf(lchild), Children::Leaf(rchild)) => lchild.extend(rchild),
						(Children::Inner(lchild), Children::Inner(rchild)) => lchild.extend(rchild),
						_ => unreachable!(),
					}
					rightmost_child.remove_rightmost()
				}
			}
		}
	}

	fn remove_leftmost(&mut self) -> T {
		match &mut self.children {
			Children::Leaf(children) => {
				let leftmost_child = children.first_mut().unwrap();
				if leftmost_child.keys.len() > MIN_LEN_AFTER_SPLIT {
					leftmost_child.remove_leftmost()
				} else if children[1].keys.len() > MIN_LEN_AFTER_SPLIT {
					// split 1 key to leftmost tree
					let new_key = std::mem::replace(
						self.keys.first_mut().unwrap(),
						children[1].keys.remove(0),
					);
					let leftmost_child = children.first_mut().unwrap();
					leftmost_child.keys.push(new_key);
					leftmost_child.remove_leftmost()
				} else {
					// merge the first two children
					let new_key = self.keys.remove(0);
					let right = children.remove(1);
					let leftmost_child = children.first_mut().unwrap();
					leftmost_child.keys.push(new_key);
					leftmost_child.keys.extend(right.keys);
					leftmost_child.remove_leftmost()
				}
			}
			Children::Inner(children) => {
				let leftmost_child = children.first_mut().unwrap();
				if leftmost_child.keys.len() > MIN_LEN_AFTER_SPLIT {
					leftmost_child.remove_leftmost()
				} else if children[1].keys.len() > MIN_LEN_AFTER_SPLIT {
					// split 1 key to leftmost tree
					let split_key = std::mem::replace(
						self.keys.first_mut().unwrap(),
						children[1].keys.remove(0),
					);
					match &mut children[1].children {
						Children::Leaf(rchildren) => {
							let split_tr = rchildren.remove(0);
							if let Children::Leaf(lchildren) = &mut children[0].children {
								lchildren.push(split_tr);
							}
						}
						Children::Inner(rchildren) => {
							let split_tr = rchildren.remove(0);
							if let Children::Inner(lchildren) = &mut children[0].children {
								lchildren.push(split_tr);
							}
						}
					}
					children[0].keys.push(split_key);
					children.first_mut().unwrap().remove_leftmost()
				} else {
					//merge first two children
					let new_key = self.keys.remove(0);
					let right = children.remove(1);
					let leftmost_child = children.first_mut().unwrap();
					leftmost_child.keys.push(new_key);
					leftmost_child.keys.extend(right.keys);
					match (&mut leftmost_child.children, right.children) {
						(Children::Leaf(lchild), Children::Leaf(rchild)) => lchild.extend(rchild),
						(Children::Inner(lchild), Children::Inner(rchild)) => lchild.extend(rchild),
						_ => unreachable!(),
					}
					leftmost_child.remove_leftmost()
				}
			}
		}
	}
}

impl<T: Ord> BTreeLeafNode<T> {
	fn find(&self, key: &T) -> Option<()> {
		match self.keys.binary_search(key) {
			Ok(_) => Some(()),
			Err(_) => None,
		}
	}

	fn insert(&mut self, key: T) -> Option<(T, Box<BTreeLeafNode<T>>)> {
		let pos = self.keys.binary_search(&key).err()?;

		if self.keys.is_full() {
			let mut split_leaf = Box::new(Self {
				keys: self.keys.drain((MIN_LEN_AFTER_SPLIT + 1)..).collect(),
			});
			let split_key = self.keys.pop().unwrap();
			if pos <= MIN_LEN_AFTER_SPLIT {
				self.keys.insert(pos, key);
			} else {
				split_leaf.keys.insert(pos - MIN_LEN_AFTER_SPLIT - 1, key);
			}
			Some((split_key, split_leaf))
		} else {
			self.keys.insert(pos, key);
			None
		}
	}

	fn remove(&mut self, key: &T) -> Option<T> {
		let pos = self.keys.binary_search(key).ok()?;
		Some(self.keys.remove(pos))
	}

	fn remove_rightmost(&mut self) -> T { self.keys.pop().unwrap() }

	fn remove_leftmost(&mut self) -> T { self.keys.remove(0) }
}

#[cfg(test)]
mod test {
	use super::*;
	use std::fmt::{Debug, Display};
	impl<T: Display> Debug for BTreeLeafNode<T> {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let mut key_it = self.keys.iter();
			if let Some(k) = key_it.next() {
				write!(f, "{}", k)?;
			}
			for k in key_it {
				write!(f, ",{}", k)?;
			}
			Ok(())
		}
	}

	impl<T: Display> Debug for BTreeInnerNode<T> {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			return fmt_inner(self, f, 0);

			fn fmt_inner<T: Display>(
				tr: &BTreeInnerNode<T>,
				f: &mut std::fmt::Formatter<'_>,
				depth: i32,
			) -> std::fmt::Result {
				match &tr.children {
					Children::Leaf(children) => {
						for (k, ch) in tr.keys.iter().zip(children) {
							for _ in 0..=depth {
								write!(f, "\t")?;
							}
							writeln!(f, "{:?}", ch)?;
							for _ in 0..depth {
								write!(f, "\t")?;
							}
							writeln!(f, "{}", k)?;
						}

						for _ in 0..=depth {
							write!(f, "\t")?;
						}
						if let Some(end) = children.get(tr.keys.len()) {
							writeln!(f, "{:?}", end)?;
						}
					}
					Children::Inner(children) => {
						for (k, ch) in tr.keys.iter().zip(children) {
							fmt_inner(ch.as_ref(), f, depth + 1)?;
							for _ in 0..depth {
								write!(f, "\t")?;
							}
							writeln!(f, "{}", k)?;
						}
						fmt_inner(children[tr.keys.len()].as_ref(), f, depth + 1)?;
					}
				}
				Ok(())
			}
		}
	}
	impl<T: Display> Debug for BTree<T> {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				Self::Leaf(root) => {
					write!(f, "{:?}", root)
				}
				Self::Inner(root) => {
					write!(f, "{:?}", root)
				}
			}
		}
	}

	fn is_sorted<T: std::cmp::Ord, I: Iterator<Item = T>>(mut it: I) -> bool {
		it.next()
			.map(|mut prev| {
				it.all(move |mut o| {
					std::mem::swap(&mut o, &mut prev);
					o <= prev
				})
			})
			.unwrap_or(true)
	}

	fn check_inner<T: Ord>(tr: &BTreeInnerNode<T>, is_root: bool) -> Result<(&T, &T), ()> {
		if !is_sorted(tr.keys.iter()) {
			dbg!("Not sorted");
			return Err(());
		}
		let mut min = tr.keys.first().unwrap();
		let max;
		if tr.keys.is_empty() || tr.keys.len() < MIN_LEN_AFTER_SPLIT && !is_root {
			dbg!("too short");
			return Err(());
		}
		let childs_len = match &tr.children {
			Children::Leaf(children) => children.len(),
			Children::Inner(children) => children.len(),
		};
		if childs_len != tr.keys.len() + 1 {
			dbg!("childs len invalid");
			return Err(());
		}

		let mut prev = None;
		match &tr.children {
			Children::Inner(children) => {
				for (k, ch) in tr.keys.iter().zip(children) {
					let (ch_min, ch_max) = check_inner(ch, false)?;
					min = min.min(ch_min);

					if let Some(prev) = prev {
						if prev > ch_min {
							dbg!("Not sorted");
							return Err(());
						}
					}
					if ch_max >= k {
						dbg!("no sorted");
						return Err(());
					}
					prev = Some(k);
				}
				let (ch_min, ch_max) = check_inner(&children[tr.keys.len()], false)?;
				if ch_min <= prev.unwrap() {
					return Err(());
				}
				max = ch_max;
			}
			Children::Leaf(children) => {
				for (k, ch) in tr.keys.iter().zip(children) {
					let (ch_min, ch_max) = check_leaf(ch, false)?;
					min = min.min(ch_min);
					if let Some(prev) = prev {
						if prev > ch_min {
							return Err(());
						}
					}
					if ch_max >= k {
						return Err(());
					}
					prev = Some(k);
				}
				let (ch_min, ch_max) = check_leaf(&children[tr.keys.len()], false)?;
				if ch_min <= prev.unwrap() {
					return Err(());
				}
				max = ch_max;
			}
		};

		Ok((min, max))
	}
	fn check_leaf<T: Ord>(tr: &BTreeLeafNode<T>, is_root: bool) -> Result<(&T, &T), ()> {
		if !is_sorted(tr.keys.iter()) {
			dbg!("Not sorted");
			return Err(());
		}
		if tr.keys.len() < MIN_LEN_AFTER_SPLIT && !is_root {
			dbg!("too short");
			return Err(());
		}

		Ok((tr.keys.first().unwrap(), tr.keys.last().unwrap()))
	}

	fn check_btree<T: Ord>(tr: &BTree<T>) -> Result<(), ()> {
		match tr {
			BTree::Inner(root) => check_inner(root, true).map(|_| ()),
			BTree::Leaf(root) => {
				if !root.keys.is_empty() {
					check_leaf(root, true).map(|_| ())
				} else {
					Ok(())
				}
			}
		}
	}

	use rand::{prelude::*, seq::SliceRandom};
	#[test]
	fn it_works() {
		let sameple = 0..100_000;
		let mut rng = rand::thread_rng();
		let rand_iter = rand::distributions::Bernoulli::from_ratio(1, 100).unwrap();
		let mut tr = BTree::default();
		for i in sameple.clone().rev() {
			tr.insert(i);
			assert!(tr.find(&i));
			if rand_iter.sample(&mut rng) {
				check_btree(&tr).unwrap();
			}
		}
		println!("{tr:?}");
		for i in sameple.clone() {
			assert!(tr.find(&i));
		}
		let mut to_del: Vec<_> = sameple.collect();
		to_del.shuffle(&mut rng);
		for i in to_del {
			assert!(tr.remove(&i).is_some());
			if rand_iter.sample(&mut rng) {
				check_btree(&tr).unwrap()
			}
			assert!(!tr.find(&i));
		}
		dbg!(&tr);
		check_btree(&tr).unwrap();
	}
}
