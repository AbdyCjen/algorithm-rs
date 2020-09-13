extern crate rand;
use std::cmp::Ordering;

struct TreapNode<T: std::cmp::Ord> {
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
	pub fn find(self: &Self, k: &T) -> Option<()> {
		self.root.as_ref()?.find(k)
	}

	pub fn ins(self: &mut Self, k: T) -> Option<()> {
		match &mut self.root {
			Some(root) => root.ins(k),
			root => root.replace(Box::new(TreapNode::new(k))).map(|_| ()),
		}
	}

	pub fn remove(self: &mut Self, _k: &T) -> Option<T> {
		unimplemented!()
	}
}

impl<T: std::cmp::Ord> TreapNode<T> {
	fn new(k: T) -> TreapNode<T> {
		TreapNode { left: None,
		            right: None,
		            r: rand::random(),
		            k }
	}

	fn by_ord(&self, ord: Ordering) -> &Option<Box<Self>> {
		match ord {
			Ordering::Less => &self.left,
			Ordering::Greater => &self.right,
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

	fn rotate_left(&mut self) {
		let mut right = self.right.take().unwrap();
		self.right = right.left.take();
		std::mem::swap(self, &mut right);
		self.left = Some(right);
	}

	fn rotate_right(&mut self) {
		let mut left = self.left.take().unwrap();
		self.left = left.right.take();
		std::mem::swap(self, &mut left);
		self.right = Some(left);
	}
	fn rotate_by_ord(&mut self, ord: Ordering) {
		match ord {
			Ordering::Less => self.rotate_right(),
			Ordering::Greater => self.rotate_left(),
			Ordering::Equal => unreachable!(),
		}
	}

	fn find(self: &Self, k: &T) -> Option<()> {
		match k.cmp(&self.k) {
			Ordering::Equal => Some(()),
			ord => self.by_ord(ord).as_ref()?.find(k),
		}
	}

	fn ins(&mut self, k: T) -> Option<()> {
		match k.cmp(&self.k) {
			Ordering::Equal => Some(()),
			ord => {
				let ret_val = match self.by_ord_mut(ord) {
					Some(ch) => ch.ins(k),
					ch => ch.replace(Box::new(Self::new(k))).map(|_| ()),
				};
				// FIXME
				if self.r < self.by_ord(ord).as_ref().unwrap().r {
					self.rotate_by_ord(ord)
				}
				ret_val
			}
		}
	}

	/*fn remove(&mut self, k: &T) -> Option<T> {
		match self.k.cmp(&k) {
			Ordering::Equal => {
				match (&self.left, &self.right) {
					(None, _) => todo!(),
					(_, None) => todo!(),
					(Some(l), Some(r)) if r.r > l.r => todo!(),
					_ => todo!(),
				}
			}
			ord => self.by_ord_mut(ord).as_mut()
				.map(|o| o.remove(k))
				.flatten(),
		}
	}*/

	/*fn remove_old(no: &mut Option<Box<Self>>, k: &T) -> Option<T> {
		let o = no.as_mut()?;
		match k.cmp(&o.k) {
<<<<<<< HEAD
			Ordering::Equal => match (&o.left, &o.right) {
				(None, _) => {
					let tmp = o.right.take();
					mem::replace(no, tmp).map(|t| t.k)
				}
				(_, None) => {
					let tmp = o.left.take();
					mem::replace(no, tmp).map(|t| t.k)
				}
				(Some(l), Some(r)) if r.r > l.r => {
					o.rotate_left();
					TreapNode::remove_old(&mut o.left, k)
				}
				_ => {
					o.rotate_right();
					TreapNode::remove_old(&mut o.right, k)
				}
			},
			ord => TreapNode::remove_old(o.by_ord_mut(ord), k),
		}
	}*/
}

// TODO: implement iterator for treap
pub struct IntoIter<T: Ord> {
	st: Vec<Box<TreapNode<T>>>,
}

impl<T: Ord> Iterator for IntoIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let mut l = self.st.pop()?;
		while let Some(lnext) = l.left.take() {
			self.st.push(l);
			l = lnext;
		}

		let mut no = l;
		if let Some(r) = no.right.take() {
			self.st.push(r);
		}
		Some(no.k)
	}
}

impl<T: Ord> IntoIterator for Treap<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		if let Some(o) = self.root {
			IntoIter { st: vec![o] }
		} else {
			IntoIter { st: Vec::new() }
		}
	}
}

pub struct Iter<'a, T: Ord> {
	st: Vec<(&'a TreapNode<T>, Ordering)>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		let (mut no, ord) = self.st.pop()?;
		if Ordering::Less == ord {
			while let Some(lnext) = &no.left {
				self.st.push((no, Ordering::Equal));
				no = lnext;
			}
		}

		if let Some(r) = &no.right {
			self.st.push((r, Ordering::Less));
		}
		Some(&no.k)
	}
}

impl<'a, T: Ord> IntoIterator for &'a Treap<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		if let Some(o) = &self.root {
			Iter { st: vec![(o, Ordering::Less)] }
		} else {
			Iter { st: Vec::new() }
		}
	}
}

pub struct IterMut<'a, T: Ord> {
	st: Vec<(&'a mut TreapNode<T>, Ordering)>,
}

impl<'a, T: Ord> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		self.st.pop()?;
		todo!()
	}
}

impl<'a, T: Ord> IntoIterator for &'a mut Treap<T> {
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		if let Some(o) = &mut self.root {
			IterMut { st: vec![(o, Ordering::Less)] }
		} else {
			IterMut { st: Vec::new() }
		}
	}
}
impl<T: Ord> Treap<T> {
	pub fn iter(&self) -> Iter<'_,T> {
		self.into_iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, T> {
		self.into_iter()
	}
}

#[cfg(test)]
mod test_treap {
	use super::*;
	impl<T> std::fmt::Debug for TreapNode<T> where T: std::fmt::Display + Ord
	{
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{{")?;
			if let Some(l) = &self.left {
				write!(f, " \"l\" : {:?},", l)?;
			}
			write!(f, " \"{}\": {}", self.k, self.r)?;
			if let Some(r) = &self.right {
				write!(f, ",\"r\" : {:?}", r)?;
			}
			write!(f, "}}")?;
			Ok(())
		}
	}

	fn check_treap<T>(t: &TreapNode<T>) -> Result<(), T>
		where T: Ord + std::fmt::Display + Copy
	{
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
	fn is_sorted<T: std::cmp::Ord, I: Iterator<Item = T>>(mut it: I) -> bool {
		if let Some(first) = it.next() {
			let mut prev = first;
			it.all(move |cur| {
				  let sorted = cur > prev;
				  prev = cur;
				  sorted
			  })
		} else {
			true
		}
	}

	#[test]
	fn test_iter() {
		const TEST_RANGE: std::ops::Range<i32> = 0..1000_000;
		let mut tr = Treap::default();
		for i in TEST_RANGE {
			tr.ins(i);
		}
		assert!(TEST_RANGE.into_iter().eq((&tr).into_iter().copied()));
		assert!(TEST_RANGE.into_iter().eq(tr.into_iter()));
	}

	#[test]
	fn it_works() {
		const TEST_RANGE: i32 = 100;
		let mut tr = Treap::default();
		for _ in 0..TEST_RANGE {
			let i = rand::random::<i32>();
			tr.ins(i);
			assert_eq!(tr.find(&i), Some(()));
		}

		//dbg!(tr.root.as_ref().unwrap());
		check_treap(&tr.root.as_ref().unwrap()).unwrap();
		assert!(is_sorted(tr.into_iter()));
	}
}
