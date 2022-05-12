use std::{cmp::Ordering, rc::Rc};

const BAL_FACTOR: u8 = 1;

#[derive(Clone)]
struct AvlNode<T> {
	key: T,
	h: u8,
	left: Option<Rc<AvlNode<T>>>,
	right: Option<Rc<AvlNode<T>>>,
}

impl<T> AvlNode<T> {
	#[inline]
	fn new(key: T) -> Self {
		Self {
			key,
			h: 1,
			left: None,
			right: None,
		}
	}

	#[inline]
	fn by_ord_all(&self, ord: Ordering) -> (Option<&Self>, Option<&Self>) {
		match ord {
			Ordering::Less => (self.left.as_deref(), self.right.as_deref()),
			Ordering::Greater => (self.right.as_deref(), self.left.as_deref()),
			_ => unreachable!(),
		}
	}

	#[inline]
	fn balance(&self, deep: Ordering) -> bool {
		let (ch, sib_ch) = self.by_ord_all(deep);
		Self::get_height(ch) <= Self::get_height(sib_ch) + BAL_FACTOR
	}

	#[inline]
	fn lean(&self, toward: Ordering) -> bool {
		let (ch, sib_ch) = self.by_ord_all(toward);
		Self::get_height(ch) >= Self::get_height(sib_ch)
	}

	#[inline]
	fn by_ord(&self, ord: Ordering) -> Option<&Self> {
		match ord {
			Ordering::Less => self.left.as_deref(),
			Ordering::Greater => self.right.as_deref(),
			_ => unreachable!(),
		}
	}

	fn get_height(root: Option<&Self>) -> u8 { root.map(|root| root.h).unwrap_or(0) }

	fn update_height(&mut self) {
		self.h = std::cmp::max(
			Self::get_height(self.left.as_deref()),
			Self::get_height(self.right.as_deref()),
		) + 1;
	}
	/*fn by_ord_take(&mut self, ord: Ordering) -> Option<Rc<AvlNode<T>>> {
		match ord {
			Ordering::Less => self.left.take(),
			Ordering::Greater => self.right.take(),
			_ => unreachable!(),
		}
	}*/
	fn by_ord_mut(&mut self, ord: Ordering) -> &mut Option<Rc<AvlNode<T>>> {
		match ord {
			Ordering::Less => &mut self.left,
			Ordering::Greater => &mut self.right,
			_ => unreachable!(),
		}
	}
}

#[derive(Default, Clone)]
pub struct AvlTree<T>(Option<Rc<AvlNode<T>>>);

impl<T: Ord> AvlNode<T> {
	pub(crate) fn find(&self, k: &T) -> Option<()> {
		match k.cmp(&self.key) {
			Ordering::Equal => Some(()),
			ord => self.by_ord(ord)?.find(k),
		}
	}

	pub(crate) fn remove(self: Rc<Self>, k: &T) -> Option<Rc<Self>> { todo!() }
}

impl<T: Ord + Clone> AvlNode<T> {
	fn rotate_by_ord(self: &mut Rc<Self>, ord: Ordering) {
		match ord {
			Ordering::Less => self.rotate_right(),
			Ordering::Greater => self.rotate_left(),
			_ => unreachable!(),
		}
	}

	fn rotate_left(self: &mut Rc<Self>) {
		let mut root = Rc::make_mut(self);
		let mut right = root.right.take().unwrap();
		let right_mut = Rc::make_mut(&mut right);
		root.right = right_mut.left.take();
		root.update_height();
		std::mem::swap(self, &mut right);

		let root = Rc::make_mut(self);
		root.left = Some(right);
		root.update_height();
	}

	fn rotate_right(self: &mut Rc<Self>) {
		// XXX:maybe too many make_mut?
		let mut root = Rc::make_mut(self);
		let mut left = root.left.take().unwrap();
		let left_mut = Rc::make_mut(&mut left);
		root.left = left_mut.left.take();
		root.update_height();
		std::mem::swap(self, &mut left);

		let root = Rc::make_mut(self);
		root.left = Some(left);
		root.update_height();
	}

	fn straighten(self: &mut Rc<Self>, toward: Ordering) {
		if !self.lean(toward) {
			self.rotate_by_ord(toward.reverse());
		}
	}

	pub(crate) fn insert(mut self: Rc<Self>, k: T) -> Rc<Self> {
		let root = Rc::make_mut(&mut self);
		match k.cmp(&root.key) {
			Ordering::Equal => self,
			ord => match root.by_ord_mut(ord) {
				ch_r @ Some(_) => {
					let ch = ch_r.take().unwrap().insert(k);
					let ch_h = ch.h;
					*ch_r = Some(ch);

					if root.h == ch_h {
						root.h += 1;
						if !root.balance(ord) {
							root.by_ord_mut(ord).as_mut().unwrap().straighten(ord);
							self.rotate_by_ord(ord);
						}
					}
					self
				}
				ch @ None => {
					ch.replace(Rc::new(AvlNode::new(k)));
					root.update_height();
					self
				}
			},
		}
	}
}

impl<T: Ord + Clone> AvlTree<T> {
	pub fn insert(self, k: T) -> Self {
		match self.0 {
			None => Self(Some(Rc::new(AvlNode::new(k)))),
			Some(root) => Self(Some(root.insert(k))),
		}
	}

	pub fn remove(self, k: &T) -> Self {
		match self.0 {
			None => self,
			Some(root) => Self(root.remove(k)),
		}
	}

	pub fn find(&self, k: &T) -> Option<()> { self.0.as_deref()?.find(k) }
}

#[cfg(test)]
mod tests {
	use super::*;

	impl<T> std::fmt::Debug for AvlNode<T>
	where
		T: std::fmt::Display + Ord,
	{
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			return inner_fmt(Some(self), f, 0);

			fn inner_fmt<T: Ord + std::fmt::Display>(
				no: Option<&AvlNode<T>>,
				f: &mut std::fmt::Formatter<'_>,
				idt_lv: usize,
			) -> std::fmt::Result {
				if let Some(no) = no {
					inner_fmt(no.left.as_deref(), f, idt_lv + 1)?;

					write!(f, "{}", "	".repeat(idt_lv))?;
					writeln!(f, "{}: {}", no.key, no.h)?;

					inner_fmt(no.right.as_deref(), f, idt_lv + 1)?;
				}
				Ok(())
			}
		}
	}

	#[test]
	fn it_works() {
		let mut trs = vec![AvlTree::default()];
		let rng = 0..2_000_000;
		for i in rng.clone() {
			trs.push(trs.last().cloned().unwrap().insert(i));
		}
		if let Some(tr) = trs.last() {
			check_avl(tr.0.as_deref().unwrap()).unwrap();
		}
	}

	fn check_avl<T: Ord + std::fmt::Display>(tr: &AvlNode<T>) -> Result<(i8, &T, &T), ()> {
		let (mut max, mut min) = (&tr.key, &tr.key);
		let (mut hl, mut hr) = (0, 0);
		if let Some(left) = tr.left.as_deref() {
			let (h, lmin, lmax) = check_avl(left)?;
			hl = h;
			if lmax > &tr.key {
				return Err(());
			}
			max = max.max(lmax);
			min = min.min(lmin);
		}

		if let Some(right) = tr.right.as_deref() {
			let (h, rmin, rmax) = check_avl(right)?;
			hr = h;
			if rmin < &tr.key {
				return Err(());
			}
			max = max.max(rmax);
			min = min.min(rmin);
		}
		if (hl - hr).abs() > BAL_FACTOR as _ {
			println!("unbal {} {}", (hl - hr), tr.key);
			return Err(());
		}
		let h = hr.max(hl) + 1;
		assert!(h == tr.h as _);
		Ok((h, min, max))
	}
}
