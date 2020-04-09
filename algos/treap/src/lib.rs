extern crate rand;
use std::{cmp::Ordering, mem};

#[derive(Debug)]
struct TreapNode<T: std::cmp::Ord> {
	r: u32,
	k: T,
	child: [Option<Box<TreapNode<T>>>; 2],
}

#[derive(Debug)]
pub enum TreapError {
	DupEntryError,
}

pub struct Treap<T: std::cmp::Ord> {
	root: Option<Box<TreapNode<T>>>,
}

impl<T: std::cmp::Ord> Treap<T> {
	pub fn new() -> Treap<T> {
		Treap { root: None }
	}
	pub fn find(self: &Self, k: &T) -> Option<()> {
		self.root.as_ref()?.find(k)
	}

	pub fn ins(self: &mut Self, k: T) -> Result<(), TreapError> {
		if self.root.is_none() {
			self.root.replace(Box::new(TreapNode::new(k)));
			Ok(())
		} else {
			TreapNode::ins(&mut self.root, k)
		}
	}

	pub fn remove(self: &mut Self, k: &T) -> Option<T> {
		TreapNode::remove(&mut self.root, k)
	}
}

impl<T: std::cmp::Ord> TreapNode<T> {
	fn new(k: T) -> TreapNode<T> {
		TreapNode {
			r: rand::random(),
			k,
			child: [None, None],
		}
	}

	fn ord2ind(ord: Ordering) -> usize {
		match ord {
			Ordering::Less => 0,
			Ordering::Greater => 1,
			Ordering::Equal => unreachable!(),
		}
	}

	fn rotate(self: &mut Box<Self>, d: usize) {
		let mut k = mem::replace(&mut self.child[d ^ 1], None).unwrap();
		self.child[d ^ 1] = mem::replace(&mut k.child[d], None);
		mem::swap(self, &mut k);
		mem::replace(&mut self.child[d], Some(k));
	}

	fn find(self: &Self, k: &T) -> Option<()> {
		match k.cmp(&self.k) {
			Ordering::Equal => Some(()),
			ord => self.child[Self::ord2ind(ord)].as_ref()?.find(k),
		}
	}

	fn ins(self_ : &mut Option<Box<Self>>, k: T) -> Result<(), TreapError> {
		match self_ {
			None => {self_.replace(Box::new(Self::new(k)));},
			Some(self_) => {
				if k == self_.k {
					return Err(TreapError::DupEntryError);
				}

				let d = Self::ord2ind(k.cmp(&self_.k));
				Self::ins(&mut self_.child[d], k)?;
				if self_.r < self_.child[d].as_ref().unwrap().r {
					self_.rotate(d ^ 1);
				}
			},
		};

		Ok(())
	}

	fn remove(no: &mut Option<Box<Self>>, k: &T) -> Option<T> {
		let o = no.as_mut()?;
		match k.cmp(&o.k) {
			Ordering::Equal => {
				if o.child[0].is_none() {
					let tmp = mem::replace(&mut o.child[1], None);
					mem::replace(no, tmp).map(|t| t.k)
				} else if o.child[1].is_none() {
					let tmp = mem::replace(&mut o.child[0], None);
					mem::replace(no, tmp).map(|t| t.k)
				} else {
					if o.child[1].as_ref()?.r > o.child[0].as_ref()?.r {
						o.rotate(1);
						TreapNode::remove(&mut o.child[1], k)
					} else {
						o.rotate(0);
						TreapNode::remove(&mut o.child[0], k)
					}
				}
			}
			ord => TreapNode::remove(&mut o.child[Self::ord2ind(ord)], k),
		}
	}
}

#[cfg(test)]
mod test_treap {
	use super::*;
	#[test]
	fn it_works() {
		let mut tr = Treap::new();
		for i in 0..1000_000 {
			tr.ins(i).unwrap()
		}
		check_treap(&tr.root.as_ref().unwrap()).unwrap();
		println!();
		tr.ins(3).unwrap_err();
	}

	fn check_treap<T>(t: &Box<TreapNode<T>>) -> Result<(), T>
	where
		T: Ord + std::fmt::Display + Copy,
	{
		if let Some(l) = t.child[0].as_ref() {
			if l.k > t.k || l.r > t.r {
				Err(t.k)?;
			}
			check_treap(l)?;
		}
		if let Some(r) = t.child[1].as_ref() {
			if r.k < t.k || r.r > t.r {
				Err(t.k)?;
			}
			check_treap(r)?;
		}
		Ok(())
	}
}
