use std::cmp::Ordering;
mod bst;
mod iterator;
use ::bst::{BSTNode, BSTNodeInner};
pub struct AvlNode<T: Ord> {
	h: i8,
	k: T,
	left: Option<Box<AvlNode<T>>>,
	right: Option<Box<AvlNode<T>>>,
}

#[derive(Default)]
pub struct AvlTree<T: Ord> {
	root: Option<Box<AvlNode<T>>>,
}

impl<T: Ord> AvlNode<T> {
	#[inline]
	fn update_height(&mut self) {
		let (l, r) = (self.left.as_deref(), self.right.as_deref());
		self.h = Self::get_height(l).max(Self::get_height(r)) + 1;
	}

	#[inline]
	fn balance(&self, deep: Ordering) -> bool {
		let (ch, sib_ch) = self.by_ord_all(deep);
		Self::get_height(ch) <= Self::get_height(sib_ch) + 1
	}

	// how to make this function take both Option<&T>, &Option<Box<T>>
	#[inline]
	fn get_height(o: Option<&Self>) -> i8 { o.map(|o| o.h).unwrap_or(0) }

	#[inline]
	fn swap_with_rightmost(&mut self, k: &mut T) {
		let mut cur = self;
		while let Some(next_no) = cur.right.as_mut() {
			cur = next_no;
		}
		std::mem::swap(&mut cur.k, k);
	}

	#[inline]
	fn straighten(&mut self, toward: Ordering) {
		if !lean_toward(self, toward) {
			self.rotate_by_ord(toward.reverse());
		}

		#[inline]
		fn lean_toward<T: Ord>(no: &AvlNode<T>, toward: Ordering) -> bool {
			let (ch, sib_ch) = no.by_ord_all(toward);
			AvlNode::get_height(ch) >= AvlNode::get_height(sib_ch)
		}
	}


	#[rustfmt::skip]
	fn insert(&mut self, k: T) -> Option<()> {
		match k.cmp(&self.k) {
			Ordering::Equal => Some(()),
			ord => match self.by_ord_mut(ord) {
				Some(ch) => {
					let to_ret = ch.insert(k);
					let ch_h = ch.h;
					if self.h == ch_h {
						self.h += 1;
						if !self.balance(ord) {
							self.by_ord_mut(ord).as_mut()?.straighten(ord);
							self.rotate_by_ord(ord)
						}
					}
					to_ret
				}
				ch @ None => {
					ch.replace(Box::new(Self::new(k)));
					self.update_height();
					None
				}
			}

		}
	}

	// FIXME: buggy
	#[rustfmt::skip]
	fn remove(root: &mut Option<Box<Self>>, k: &T) -> Option<()> {
		let no = root.as_mut()?;
		match k.cmp(&no.k) {
			Ordering::Equal => match (&mut no.left, &mut no.right) {
				(None, ch) | (ch, None) => {
					*root = ch.take();
					Some(())
				}

				(Some(ch), _) => {
					ch.swap_with_rightmost(&mut no.k);
					Self::remove(&mut no.left, k);
					no.update_height();
					// rebal
					if !no.balance(Ordering::Greater) {
						no.right.as_mut()?.straighten(Ordering::Greater);
						no.rotate_left();
					}

					Some(())
				}
			}

			ord => {
				let to_ret = Self::remove(no.by_ord_mut(ord), k);
				no.update_height();
				let ord_rev = ord.reverse();
				if !no.balance(ord_rev) {
					no.by_ord_mut(ord_rev).as_mut()?.straighten(ord_rev);
					no.rotate_by_ord(ord_rev);
				}

				to_ret
			}
		}
	}

	fn new(k: T) -> AvlNode<T> {
		AvlNode {
			h: 1,
			k,
			left: None,
			right: None,
		}
	}
}

impl<T: Ord> AvlTree<T> {
	pub fn insert(&mut self, k: T) -> Option<()> {
		match &mut self.root {
			Some(o) => o.insert(k),
			empty_root => empty_root.replace(Box::new(AvlNode::new(k))).map(|_| {}),
		}
	}

	pub fn remove(&mut self, k: &T) -> Option<()> { AvlNode::remove(&mut self.root, k) }
}

#[cfg(test)]
mod tests {
	use super::*;
	use ::bst::BSTree;
	fn check_avl<T: Ord>(root: &AvlNode<T>) -> (i8, bool) {
		let check_rec =
			|no: &Option<Box<AvlNode<T>>>| no.as_deref().map(check_avl).unwrap_or((0, true));
		let (lh, mut lb) = check_rec(&root.left);
		let (rh, mut rb) = check_rec(&root.right);
		lb &= root.left.as_deref().map(|o| root.k > o.k).unwrap_or(true);
		rb &= root.right.as_deref().map(|o| root.k < o.k).unwrap_or(true);
		let cb = (lh - rh).abs() <= 1;
		let h = 1 + lh.max(rh);
		(h, lb && rb && cb && h == root.h)
	}

	impl<T> std::fmt::Debug for AvlNode<T>
	where T: std::fmt::Display + Ord
	{
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			return inner_fmt(Some(self), f, 0);

			fn inner_fmt<T: Ord + std::fmt::Display>(
				no: Option<&AvlNode<T>>,
				f: &mut std::fmt::Formatter<'_>,
				idt_lv: usize,
			) -> std::fmt::Result
			{
				if let Some(no) = no {
					inner_fmt(no.left.as_deref(), f, idt_lv + 1)?;

					write!(f, "{}", "	".repeat(idt_lv))?;
					writeln!(f, "{}: {}", no.k, no.h)?;

					inner_fmt(no.right.as_deref(), f, idt_lv + 1)?;
				}
				Ok(())
			}
		}
	}
	#[test]
	fn it_works() {
		let mut tr = AvlTree::default();
		//const TEST_RANGE: std::ops::Range<i32> = 0..100;
		const TEST_RANGE: std::ops::Range<i32> = 0..1_000_000;
		let mut hs = std::collections::HashSet::new();
		for i in TEST_RANGE.map(|_| rand::random::<i32>()) {
			hs.insert(i);
			tr.insert(i);
		}
		for i in hs.iter() {
			assert!(tr.find(i).is_some());
		}
		let (th, avl_ok) = check_avl(tr.root.as_deref().unwrap());
		assert!(avl_ok);
		println!("tr height {}, cnt: {}", th, hs.len());
		for i in hs.iter() {
			if rand::random::<i32>() % 100000 == 1 {
				let snap_shot = format!("{:?}", tr.root.as_deref().unwrap());
				if tr.remove(i).is_none()
					|| !tr.root.as_deref().map(|o| check_avl(o).1).unwrap_or(true)
				{
					println!("{} \n{}", i, snap_shot);
					println!("after:\n{:?}", tr.root.as_deref().unwrap());
					panic!();
				}
				println!("random check OK");
			} else {
				assert!(tr.remove(i).is_some());
			}
		}
	}
}
