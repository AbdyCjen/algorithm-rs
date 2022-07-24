use super::*;
pub struct Iter<'a, T> {
	st: Vec<&'a AvlNode<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		let top = self.st.pop()?;

		if let Some(mut next_node) = top.right.as_ref() {
			self.st.push(next_node);
			while let Some(left) = next_node.left.as_ref() {
				next_node = left;
				self.st.push(next_node);
			}
		}

		Some(&top.key)
	}
}

impl<'a, T> IntoIterator for &'a AvlTree<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		let mut st = Vec::new();
		let mut cur = &self.0;
		while let Some(no) = cur {
			st.push(no.as_ref());
			cur = &no.left;
		}

		Iter { st }
	}
}

impl<T: Ord + Clone> FromIterator<T> for AvlTree<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		iter.into_iter().fold(Self::default(), |tr, v| tr.insert(v))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn iter_test() {
		let vec: Vec<_> = (0..100).collect();
		let tr: AvlTree<i32> = vec.iter().copied().collect();
		let new_vec: Vec<_> = (&tr).into_iter().copied().collect();
		assert_eq!(vec, new_vec);
	}
}
