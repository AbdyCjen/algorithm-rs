use std::cmp::{Ord, Ordering};
mod iterator;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RB {
	Red,
	Black,
}
use RB::*;
pub struct RbTreeNode<T: Ord> {
	left: Option<Box<RbTreeNode<T>>>,
	right: Option<Box<RbTreeNode<T>>>,
	k: T,
	color: RB,
}

#[derive(Default)]
pub struct RBTree<T: Ord> {
	root: Option<RbTreeNode<T>>,
}

impl<T: Ord> RBTree<T> {
	pub fn find(&self, k: &T) -> Option<()> {
		self.root.as_ref()?.find(k)
	}

	pub fn insert(&mut self, k: T) -> Option<()> {
		match &mut self.root {
			Some(root) => {
				let res = root.insert(k);
				root.color = Black;
				res
			}
			root => root.replace(RbTreeNode::new(k)).map(|_| ()), // FIXME: root.color == Red
		}
	}

	pub fn remove(&mut self, _k: &T) -> Option<()> {
		unimplemented!()
	}
}

impl<T: Ord> RbTreeNode<T> {
	#[inline]
	fn by_ord(&self, ord: Ordering) -> Option<&Self> {
		match ord {
			Ordering::Less => self.left.as_ref().map(AsRef::as_ref),
			Ordering::Greater => self.right.as_ref().map(AsRef::as_ref),
			_ => unreachable!(),
		}
	}

	#[inline]
	fn by_ord_mut(&mut self, ord: Ordering) -> &mut Option<Box<Self>> {
		match ord {
			Ordering::Less => &mut self.left,
			Ordering::Greater => &mut self.right,
			_ => unreachable!(),
		}
	}

	#[inline]
	fn by_ord_mut_all(&mut self,
	                  ord: Ordering)
	                  -> (&mut Option<Box<Self>>, &mut Option<Box<Self>>) {
		match ord {
			Ordering::Less => (&mut self.left, &mut self.right),
			Ordering::Greater => (&mut self.right, &mut self.left),
			_ => unreachable!(),
		}
	}

	#[inline]
	fn new(k: T) -> RbTreeNode<T> {
		RbTreeNode { left: None,
		             right: None,
		             k,
		             color: Red }
	}

	fn find(&self, k: &T) -> Option<()> {
		match k.cmp(&self.k) {
			Ordering::Equal => Some(()),
			ord => self.by_ord(ord)?.find(k),
		}
	}

	#[inline]
	fn flip_color(&mut self) {
		self.color = Red;
		self.left.as_mut().unwrap().color = Black;
		self.right.as_mut().unwrap().color = Black;
	}

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

	// make targeted child be the new root
	#[inline]
	fn rotate_by_ord(&mut self, ord: Ordering) {
		match ord {
			Ordering::Less => self.rotate_right(),
			Ordering::Greater => self.rotate_left(),
			_ => unreachable!(),
		}
	}

	fn insert(&mut self, k: T) -> Option<()> {
		// only node with red link matters
		match k.cmp(&self.k) {
			Ordering::Equal => Some(()),
			ord => match self.by_ord_mut_all(ord) {
				(Some(ch), other_ch) if ch.color == Red => match k.cmp(&ch.k) {
					Ordering::Equal => Some(()),
					sub_ord => {
						let ret_val = match ch.by_ord_mut(sub_ord) {
							Some(ch_ch) => ch_ch.insert(k),
							ch_ch => ch_ch.replace(Box::new(Self::new(k))).map(|_| ()),
						};

						// the other child is also red
						if ch.by_ord(sub_ord).unwrap().color == Red {
							if other_ch.as_ref().map(|o| o.color) == Some(Red) {
								self.flip_color();
							} else {
								if ord != sub_ord {
									ch.rotate_by_ord(sub_ord);
								}
								self.rotate_by_ord(ord);
							}
						}
						ret_val
					}
				},
				(Some(ch), _) => ch.insert(k),
				//XXX: what if ch become red after insert ==> if we make sure `self` is not red originally, then it's ok;
				(ch @ None, _) => ch.replace(Box::new(Self::new(k))).map(|_| ()),
			},
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use rand;
	impl<T> std::fmt::Debug for RbTreeNode<T> where T: std::fmt::Display + Ord
	{
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{{")?;
			if let Some(l) = &self.left {
				write!(f, " \"l\" : {:?},", l)?;
			}
			write!(f, " \"{}\": \"{:?}\"", self.k, self.color)?;
			if let Some(r) = &self.right {
				write!(f, ",\"r\" : {:?}", r)?;
			}
			write!(f, "}}")?;
			Ok(())
		}
	}

	fn check_rbtree<T: Ord>(root: &RbTreeNode<T>) -> Result<(i32, i32), &str> {
		let (lbh, lh) = root.left
		                    .as_ref()
		                    .map(|l| {
			                    if root.color == Red && l.color == Red {
				                    return Err("dup red link");
			                    }
			                    if root.k <= l.k {
				                    return Err("left child with larger key");
			                    }
			                    Ok(check_rbtree(l)?)
		                    })
		                    .unwrap_or(Ok((0, 0)))?;

		let (rbh, rh) = root.right
		                    .as_ref()
		                    .map(|r| {
			                    if root.color == Red && r.color == Red {
				                    return Err("dup red link");
			                    }
			                    if root.k >= r.k {
				                    return Err("right child with smaller key");
			                    }
			                    Ok(check_rbtree(r)?)
		                    })
		                    .unwrap_or(Ok((0, 0)))?;

		if lbh != rbh {
			return Err("unbalance");
		}

		let h = std::cmp::max(lh, rh) + 1;
		if root.color == Black {
			Ok((lbh + 1, h))
		} else {
			Ok((lbh, h))
		}
	}
	#[test]
	fn it_works() {
		let mut rbt: RBTree<_> = Default::default();
		const TEST_RANGE: i32 = 1_000_000;
		for _ in 0..TEST_RANGE {
			rbt.insert(rand::random::<i32>());
		}
		println!(
		         "tree (black height, height) is: {:?}",
		         check_rbtree(rbt.root.as_ref().unwrap()).unwrap()
		);
	}
}
