use crate::*;
use ::bst::derive_bst;

derive_bst! {Treap, TreapNode;
	fn rotate_left(&mut self) {
		let mut right = self.right.take().unwrap();
		self.right = right.left.take();
		std::mem::swap(self, &mut right);
		self.left = Some(right);
	};
	fn rotate_right(&mut self) {
		let mut left = self.left.take().unwrap();
		self.left = left.right.take();
		std::mem::swap(self, &mut left);
		self.right = Some(left);
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use bst_util::bst_tests::bst_valid;
	const TEST_RANGE: std::ops::Range<i32> = 0..1_000_0;
	#[test]
	fn bst_test() {
		let mut rbt: Treap<_> = Default::default();
		let mut test_case = Vec::new();
		for _ in TEST_RANGE {
			test_case.push(rand::random::<i32>());
		}
		for i in test_case.iter().copied() {
			rbt.insert(i);
		}

		bst_valid(&rbt);
		for i in &test_case {
			assert!(rbt.find(i).is_some());
		}
	}
}
