/**
 * [321] Create Maximum Number
 *
 * You are given two integer arrays nums1 and nums2 of lengths m and n respectively. nums1 and nums2 represent the digits of two numbers. You are also given an integer k.
 * Create the maximum number of length k <= m + n from digits of the two numbers. The relative order of the digits from the same array must be preserved.
 * Return an array of the k digits representing the answer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums1 = [3,4,6,5], nums2 = [9,1,2,5,8,3], k = 5
 * Output: [9,8,6,5,3]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums1 = [6,7], nums2 = [6,0,4], k = 5
 * Output: [6,7,6,0,4]
 *
 * <strong class="example">Example 3:
 *
 * Input: nums1 = [3,9], nums2 = [8,9], k = 3
 * Output: [9,8,9]
 *
 *  
 * Constraints:
 *
 *     m == nums1.length
 *     n == nums2.length
 *     1 <= m, n <= 500
 *     0 <= nums1[i], nums2[i] <= 9
 *     1 <= k <= m + n
 *
 */
pub struct Solution {}

// submission codes start here
use std::rc::Rc;
#[derive(PartialEq, Eq, Debug, Ord, PartialOrd)]
struct List(i32, Option<Rc<Self>>);
impl Solution {
	pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
		let mut cache1 = vec![vec![None; k as usize + 1]; nums1.len() + 1];
		let mut cache2 = vec![vec![None; k as usize + 1]; nums2.len() + 1];
		let mut ans = vec![];
		for i in 0.max(k - nums2.len() as i32)..=k.min(nums1.len() as i32) {
			ans = ans.max(Self::merge(
				&Self::solve(&nums1, i, &mut cache1),
				&Self::solve(&nums2, k - i, &mut cache2),
			));
		}
		ans
	}

	// FIXME: better way without `List` maybe?
	fn solve(nums: &[i32], k: i32, cache: &mut [Vec<Option<Rc<List>>>]) -> Option<Rc<List>> {
		if nums.len() < k as usize || k == 0 {
			return None;
		} else if let Some(a) = &cache[nums.len()][k as usize] {
			return Some(a.clone());
		}
		let (a, r) = (nums[0], &nums[1..]);
		let ans = Self::solve(r, k, cache).max(Some(List(a, Self::solve(r, k - 1, cache)).into()));
		cache[nums.len()][k as usize].clone_from(&ans);
		ans
	}

	fn merge(mut l: &Option<Rc<List>>, mut r: &Option<Rc<List>>) -> Vec<i32> {
		let mut ans = vec![];
		loop {
			match (l, r) {
				(Some(ll), Some(rr)) if ll > rr => {
					ans.push(ll.0);
					l = &ll.1;
				}
				(_, Some(rr)) => {
					ans.push(rr.0);
					r = &rr.1;
				}
				(Some(ll), _) => {
					l = &ll.1;
					ans.push(ll.0);
				}
				(None, None) => break ans,
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_321() {
		assert_eq!(
			Solution::max_number(vec![6, 7], vec![6, 0, 4], 5),
			vec![6, 7, 6, 0, 4]
		);
	}
}
