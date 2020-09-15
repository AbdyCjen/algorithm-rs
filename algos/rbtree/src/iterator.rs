use crate::*;
pub struct IntoIter<T: Ord> {
	st: Vec<Box<RbTreeNode<T>>>,
}

impl<T: Ord> Iterator for IntoIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		let mut l = self.st.pop()?;
		while let Some(lnext) = l.left.take() {
			self.st.push(l);
			l = lnext;
		}

		if let Some(r) = l.right.take() {
			self.st.push(r);
		}
		Some(l.k)
	}
}

impl<T: Ord> IntoIterator for RBTree<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		if let Some(o) = self.root {
			IntoIter { st: vec![Box::new(o)] }
		} else {
			IntoIter { st: Vec::new() }
		}
	}
}

pub struct Iter<'a, T: Ord> {
	st: Vec<(&'a RbTreeNode<T>, Ordering)>,
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

impl<'a, T: Ord> IntoIterator for &'a RBTree<T> {
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

#[cfg(test)]
mod test{
	use super::*;
	#[test]
	fn test_iter() {
		let mut rbt: RBTree<_> = Default::default();
		const TEST_RANGE: i32 = 1_000_000;
		for i in 0..TEST_RANGE {
			rbt.insert(i);
		}
		assert!((0..TEST_RANGE).eq(rbt.into_iter()));
	}
}
