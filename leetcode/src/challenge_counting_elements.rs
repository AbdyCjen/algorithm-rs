#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashSet;
#[allow(dead_code)]
impl Solution {
	pub fn count_elements(arr: Vec<i32>) -> i32 {
		let mut hs = HashSet::new();
		for n in &arr {
			hs.insert(n);
		}
		arr.iter()
			.fold(0, |acc, n| acc + if hs.contains(&(n + 1)) { 1 } else { 0 })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_counting_elements() {
		assert_eq!(Solution::count_elements(vec![1, 2, 3]), 2);
		assert_eq!(Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]), 0);
		assert_eq!(Solution::count_elements(vec![1, 3, 2, 3, 5, 0]), 3);
		assert_eq!(Solution::count_elements(vec![1, 1, 2, 2]), 2);
	}
}
