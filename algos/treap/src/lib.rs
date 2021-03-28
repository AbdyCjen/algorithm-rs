extern crate rand;
use ::bst::BstNodeInner;
use std::cmp::Ordering;
mod bst;

pub struct TreapNode<T: std::cmp::Ord> {
	left: Option<Box<TreapNode<T>>>,
	right: Option<Box<TreapNode<T>>>,
	r: u32,
	k: T,
}

#[derive(Default)]
pub struct Treap<T: std::cmp::Ord> {
	root: Option<Box<TreapNode<T>>>,
}

impl<T: std::cmp::Ord> Treap<T> {
	pub fn find(&self, k: &T) -> Option<()> { self.root.as_ref()?.find(k) }

	pub fn insert(&mut self, k: T) -> Option<()> {
		match &mut self.root {
			Some(root) => root.insert(k),
			root => root.replace(Box::new(TreapNode::new(k))).map(|_| ()),
		}
	}

	pub fn remove(&mut self, k: &T) -> Option<()> { TreapNode::<_>::remove(&mut self.root, k) }
}

impl<T: std::cmp::Ord> TreapNode<T> {
	fn new(k: T) -> TreapNode<T> {
		TreapNode {
			left: None,
			right: None,
			r: rand::random(),
			k,
		}
	}

	fn by_ord(&self, ord: Ordering) -> Option<&Self> {
		match ord {
			Ordering::Less => self.left.as_deref(),
			Ordering::Greater => self.right.as_deref(),
			Ordering::Equal => unreachable!(),
		}
	}

	fn by_ord_mut(&mut self, ord: Ordering) -> &mut Option<Box<Self>> {
		match ord {
			Ordering::Less => &mut self.left,
			Ordering::Greater => &mut self.right,
			Ordering::Equal => unreachable!(),
		}
	}

	fn find(&self, k: &T) -> Option<()> {
		match k.cmp(&self.k) {
			Ordering::Equal => Some(()),
			ord => self.by_ord(ord)?.find(k),
		}
	}

	fn insert(&mut self, k: T) -> Option<()> {
		match k.cmp(&self.k) {
			Ordering::Equal => Some(()),
			ord => {
				let ret_val = match self.by_ord_mut(ord) {
					Some(ch) => ch.insert(k),
					ch => ch.replace(Box::new(Self::new(k))).map(|_| ()),
				};
				// FIXME
				if self.r < self.by_ord(ord).unwrap().r {
					self.rotate_by_ord(ord)
				}
				ret_val
			}
		}
	}

	#[rustfmt::skip]
	fn remove(node: &mut Option<Box<Self>>, k: &T) -> Option<()> {
		let node_inner = node.as_mut()?;
		match k.cmp(&node_inner.k) {
			Ordering::Equal => {
				match (&node_inner.left, &node_inner.right) {
					(None, _) => *node = node_inner.right.take(),
					(_, None) => *node = node_inner.left.take(),
					(Some(l), Some(r)) => if r.r > l.r {
						node_inner.rotate_left();
						return Self::remove(&mut node_inner.left, k);
					} else {
						node_inner.rotate_right();
						return Self::remove(&mut node_inner.right, k);
					}
				};
				Some(())
			}
			ord => Self::remove(node_inner.by_ord_mut(ord), k),
		}
	}
}

#[cfg(test)]
mod test_treap {
	use super::*;
	impl<T> std::fmt::Debug for TreapNode<T>
	where T: std::fmt::Display + Ord
	{
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			return inner_fmt(Some(self), f, 0);

			fn inner_fmt<T: Ord + std::fmt::Display>(
				no: Option<&TreapNode<T>>,
				f: &mut std::fmt::Formatter<'_>,
				idt_lv: usize,
			) -> std::fmt::Result
			{
				if let Some(no) = no {
					inner_fmt(no.left.as_deref(), f, idt_lv + 1)?;

					write!(f, "{}", "	".repeat(idt_lv))?;
					writeln!(f, "{}: {}", no.k, no.r)?;

					inner_fmt(no.right.as_deref(), f, idt_lv + 1)?;
				}
				Ok(())
			}
		}
	}

	fn check_treap<T>(t: &TreapNode<T>) -> Result<(), T>
	where T: Ord + std::fmt::Display + Copy {
		if let Some(l) = t.left.as_ref() {
			if l.k >= t.k || l.r > t.r {
				return Err(t.k);
			}
			check_treap(l)?;
		}
		if let Some(r) = t.right.as_ref() {
			if r.k <= t.k || r.r > t.r {
				return Err(t.k);
			}
			check_treap(r)?;
		}
		Ok(())
	}

	#[test]
	fn it_works() {
		const TEST_RANGE: std::ops::Range<i32> = 0..1000;
		let mut tr = Treap::default();
		let mut input_seq = std::collections::HashSet::new();
		for _ in TEST_RANGE {
			let i = rand::random::<i32>() % 10000;
			tr.insert(i);
			input_seq.insert(i);
		}

		for i in &input_seq {
			assert!(tr.find(i).is_some());
		}

		check_treap(&tr.root.as_ref().unwrap()).unwrap();
		//dbg!(tr.root.as_mut().unwrap());

		for i in &input_seq {
			assert!(tr.remove(i).is_some());
			assert!(tr.find(i).is_none());
		}
	}
}
