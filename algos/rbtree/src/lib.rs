use ::bst::{BstNode, BstNodeInner};
use std::cmp::{Ord, Ordering};
mod bst;
mod iterator;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rb {
	Red,
	Black,
}
use Rb::*;
pub struct RbTreeNode<T: Ord> {
	left: Option<Box<RbTreeNode<T>>>,
	right: Option<Box<RbTreeNode<T>>>,
	k: T,
	color: Rb,
}

#[derive(Default)]
pub struct RbTree<T: Ord> {
	root: Option<Box<RbTreeNode<T>>>,
}

impl<T: Ord> RbTree<T> {
	fn insert(&mut self, k: T) -> Option<()> {
		match &mut self.root {
			Some(root) => {
				let res = root.insert(k);
				root.color = Black;
				res
			}
			root => root.replace(Box::new(RbTreeNode::new(k))).map(|_| ()), // FIXME: root.color == Red
		}
	}

	#[allow(dead_code, unused_variables)]
	fn remove(&mut self, k: &T) -> Option<()> { unimplemented!() }
}

impl<T: Ord> RbTreeNode<T> {
	#[inline]
	fn by_ord_mut_all(
		&mut self,
		ord: Ordering,
	) -> (&mut Option<Box<Self>>, &mut Option<Box<Self>>)
	{
		match ord {
			Ordering::Less => (&mut self.left, &mut self.right),
			Ordering::Greater => (&mut self.right, &mut self.left),
			_ => unreachable!(),
		}
	}

	#[inline]
	fn new(k: T) -> RbTreeNode<T> {
		RbTreeNode {
			left: None,
			right: None,
			k,
			color: Red,
		}
	}

	#[inline]
	fn flip_color(&mut self) {
		self.color = Red;
		self.left.as_mut().unwrap().color = Black;
		self.right.as_mut().unwrap().color = Black;
	}

	#[inline]
	fn is_red(&self) -> bool { self.color == Red }

	fn insert(&mut self, k: T) -> Option<()> {
		// only node with red link matters
		let ord = k.cmp(&self.k);
		if ord == Ordering::Equal {
			return Some(());
		}

		match self.by_ord_mut_all(ord) {
			(Some(ch), other_ch) if ch.is_red() => {
				let sub_ord = k.cmp(&ch.k);
				if sub_ord == Ordering::Equal {
					return Some(());
				}

				let ret_val = match ch.by_ord_mut(sub_ord) {
					Some(ch_ch) => ch_ch.insert(k),
					ch_ch => ch_ch.replace(Box::new(Self::new(k))).map(|_| ()),
				};

				if ch.by_ord(sub_ord).unwrap().is_red() {
					// the other child is also red
					if other_ch.as_ref().map(|o| o.is_red()).unwrap_or(false) {
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
			(Some(ch), _) => ch.insert(k),
			//XXX: what if ch become red after insert ==> if we make sure `self` is not red originally, then it's ok;
			(ch @ None, _) => ch.replace(Box::new(Self::new(k))).map(|_| ()),
		}
	}

	#[allow(dead_code, unused_variables)]
	fn remove(node: &mut Option<Box<RbTreeNode<T>>>, k: &T) -> Option<()> { unimplemented!() }
}

#[cfg(test)]
mod tests {
	use super::*;
	use ::bst::BsTree;
	//use rand;
	impl<T> std::fmt::Debug for RbTreeNode<T>
	where T: std::fmt::Display + Ord
	{
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			return inner_fmt(Some(self), f, 0);

			fn inner_fmt<T: Ord + std::fmt::Display>(
				no: Option<&RbTreeNode<T>>,
				f: &mut std::fmt::Formatter<'_>,
				idt_lv: usize,
			) -> std::fmt::Result
			{
				if let Some(no) = no {
					inner_fmt(no.left.as_deref(), f, idt_lv + 1)?;

					write!(f, "{}", "	".repeat(idt_lv))?;
					writeln!(f, "{}: {}", no.k, if no.is_red() { 'R' } else { 'B' })?;

					inner_fmt(no.right.as_deref(), f, idt_lv + 1)?;
				}
				Ok(())
			}
		}
	}

	fn check_rbtree<T: Ord>(root: &RbTreeNode<T>) -> Result<(i32, i32), &str> {
		let check_subtree = |ord: Ordering| {
			root.by_ord(ord).map(|o| {
				if root.is_red() && o.is_red() {
					return Err("dup red link");
				}

				// XXX: check it out
				if root.k.cmp(&o.k) == ord {
					return Err("invalid bst");
				}
				Ok(check_rbtree(o)?)
			})
		};
		let (lbh, lh) = check_subtree(Ordering::Less).unwrap_or(Ok((0, 0)))?;
		let (rbh, rh) = check_subtree(Ordering::Greater).unwrap_or(Ok((0, 0)))?;

		if lbh != rbh {
			return Err("unbalance");
		}

		let h = std::cmp::max(lh, rh) + 1;
		if root.is_red() {
			Ok((lbh, h))
		} else {
			Ok((lbh + 1, h))
		}
	}
	#[test]
	fn it_works() {
		let mut rbt: RbTree<_> = Default::default();
		let mut test_seq = Vec::new();
		const TEST_RANGE: std::ops::Range<i32> = 0..1000_000;
		for _ in TEST_RANGE {
			let num = rand::random::<i32>() % TEST_RANGE.end;
			test_seq.push(num);
			rbt.insert(num);
		}
		println!(
			"tree (black height, height) is: {:?}",
			check_rbtree(rbt.root_ref().unwrap()).unwrap()
		);

		// Removal test
		/*println!("{:?}", &rbt.root.as_ref().unwrap());
		for k in &test_seq[..1] {
			rbt.remove(k);
			let test_res = check_rbtree(rbt.root.as_ref().unwrap());
			match test_res {
				Ok(o) => println!("tree (black height, height) is: {:?}", o),
				Err(e) => {
					println!("{}", e);
					println!("{:?}", rbt.root.as_ref().unwrap());
					panic!();
				}
			}
		}*/
	}
}
