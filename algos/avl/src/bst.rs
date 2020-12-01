use crate::*;
use ::bst::derive_bst;
derive_bst! {AvlTree, AvlNode;
	fn rotate_left(&mut self) {
		// FIXME: height
		let mut right = self.right.take().unwrap();
		self.right = right.left.take();
		self.update_height();
		std::mem::swap(self, &mut right);
		self.left = Some(right);
		self.update_height();
	};
	fn rotate_right(&mut self) {
		// FIXME: height
		let mut left = self.left.take().unwrap();
		self.left = left.right.take();
		self.update_height();
		std::mem::swap(self, &mut left);
		self.right = Some(left);
		self.update_height();
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use bst_util::bst_tests::*;
	#[test]
	fn bst_basic() {
		let mut tr: AvlTree<_> = Default::default();
		let mut test_case = Vec::new();
		const TEST_RANGE: std::ops::Range<i32> = 0..1_000_000;
		for _ in TEST_RANGE {
			test_case.push(rand::random::<i32>());
		}

		for i in test_case.iter().copied() {
			tr.insert(i);
		}
		bst_valid(&tr);

		for i in &test_case {
			assert!(tr.find(i).is_some());
		}
	}
}
