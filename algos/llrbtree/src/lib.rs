use std::cmp::{Ord, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rb {
	Red,
	Black,
}
use Rb::*;
struct LlrbTreeNode<T: Ord> {
	left: Option<Box<LlrbTreeNode<T>>>,
	right: Option<Box<LlrbTreeNode<T>>>,
	k: T,
	color: Rb,
}

#[derive(Default)]
pub struct RbTree<T: Ord> {
	root: Option<LlrbTreeNode<T>>,
}

impl<T: Ord> RbTree<T> {
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
			root => {
				*root = Some(LlrbTreeNode::new(k));
				None // FIXME:: root.color == Red
			}
		}
	}

	pub fn remove(&mut self, _k: &T) -> Option<()> {
		unimplemented!()
	}
}

impl<T: Ord> LlrbTreeNode<T> {
	fn by_ord(&self, ord: Ordering) -> &Option<Box<Self>> {
		match ord {
			Ordering::Less => &self.left,
			Ordering::Greater => &self.right,
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
	fn new(k: T) -> LlrbTreeNode<T> {
		LlrbTreeNode { left: None,
		               right: None,
		               k,
		               color: Red }
	}

	fn find(&self, k: &T) -> Option<()> {
		match self.k.cmp(k) {
			Ordering::Equal => Some(()),
			ord => self.by_ord(ord).as_ref()?.find(k),
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

	#[inline]
	fn insert_into_3node(&mut self, k: T) -> Option<()> {
		match self.k.cmp(&k) {
			Ordering::Less => {
				let left_child = self.left.as_mut().unwrap();
				match left_child.k.cmp(&k) {
					Ordering::Equal => Some(()),
					ord => {
						let mut child = left_child.by_ord_mut(ord);
						let ret_val = match &mut child {
							Some(o) => o.insert(k),
							child => {
								**child = Some(Box::new(Self::new(k)));
								None
							}
						};

						let child = child.as_mut().unwrap();
						if Red == child.color {
							if ord == Ordering::Greater {
								left_child.rotate_left();
							}
							self.rotate_right();
							self.flip_color();
						}
						ret_val
					}
				}
			}
			Ordering::Greater => {
				let ret_val = match &mut self.right {
					Some(o) => o.insert(k),
					right_child => {
						*right_child = Some(Box::new(Self::new(k)));
						None
					}
				};

				if self.right.as_ref().unwrap().color == Red {
					self.flip_color();
				}
				ret_val
			}
			_ => Some(()),
		}
	}

	fn insert(&mut self, k: T) -> Option<()> {
		// only 3-node and 2-node to-right matters
		if Some(Red) != self.left.as_ref().map(|o| o.color) {
			match self.k.cmp(&k) {
				Ordering::Equal => Some(()),
				ord => match self.by_ord_mut(ord) {
					Some(o) => o.insert(k),
					child => {
						*child = Some(Box::new(Self::new(k)));
						if ord == Ordering::Greater {
							self.rotate_left();
						}
						None
					}
				},
			}
		} else {
			self.insert_into_3node(k)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	impl<T> std::fmt::Debug for LlrbTreeNode<T> where T: std::fmt::Display + Ord
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

	fn check_rbtree<T: Ord>(root: &LlrbTreeNode<T>) -> Result<i32, &str> {
		let lh = root.left
		             .as_ref()
		             .map(|l| {
			             if root.color == Red && l.color == Red {
				             return Err("dup red link");
			             }
			             Ok(check_rbtree(l)?)
		             })
		             .unwrap_or(Ok(0))?;

		let rh = root.right
		             .as_ref()
		             .map(|r| {
			             if r.color == Red {
				             return Err("invalid right red link");
			             }
			             Ok(check_rbtree(r)?)
		             })
		             .unwrap_or(Ok(0))?;

		if lh != rh {
			return Err("unbalance");
		}

		if root.color == Black {
			Ok(lh + 1)
		} else {
			Ok(lh)
		}
	}
	#[test]
	fn it_works() {
		let mut rbt: RbTree<_> = Default::default();
		for i in 0..1000_000 {
			rbt.insert(i);
		}

		println!(
		         "tree black height {}",
		         check_rbtree(rbt.root.as_ref().unwrap()).unwrap()
		);
		//dbg!(rbt.root.as_ref().unwrap());
		for i in 0..1000_000 {
			assert_eq!(rbt.find(&i), Some(()));
		}
	}
}
