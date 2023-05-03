/**
 * [239] Sliding Window Maximum
 *
 * Given an array nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position. Return the max sliding window.
 *
 * Example:
 *
 *
 * Input: nums = [1,3,-1,-3,5,3,6,7], and k = 3
 * Output: [3,3,5,5,6,7]
 * Explanation:
 *
 * Window position                Max
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 *  1 [3  -1  -3] 5  3  6  7       3
 *  1  3 [-1  -3  5] 3  6  7       5
 *  1  3  -1 [-3  5  3] 6  7       5
 *  1  3  -1  -3 [5  3  6] 7       6
 *  1  3  -1  -3  5 [3  6  7]      7
 *
 *
 * Note: <br />
 * You may assume k is always valid, 1 &le; k &le; input array's size for non-empty array.
 *
 * Follow up:<br />
 * Could you solve it in linear time?
 */
pub struct Solution {}

// submission codes start here

use std::collections::BTreeMap;
impl Solution {
	pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
		if k == 1 {
			return nums;
		}
		let mut prv = 0;
		let len = nums.len();
		let mut to_left = vec![0; len];
		for ((&n, m), i) in nums.iter().zip(&mut to_left).zip(0..) {
			if i % k == 0 {
				prv = n;
			} else {
				prv = prv.max(n);
			}
			*m = prv;
		}
		let mut to_right = nums;
		let mut prv = i32::MIN;
		for (m, i) in to_right.iter_mut().zip(0..len as i32).rev() {
			if i % k == 0 {
				prv = *m;
			} else {
				prv = prv.max(*m);
			}
			*m = prv;
		}

		let mut ans = vec![0; len + 1 - k as usize];
		for (m, i) in ans.iter_mut().zip(0..) {
			*m = to_right[i].max(to_left[i + k as usize - 1]);
		}
		ans
	}
	pub fn max_sliding_window1(nums: Vec<i32>, k: i32) -> Vec<i32> {
		if k == 1 {
			return nums;
		}
		let mut cnts = BTreeMap::new();

		for &i in &nums[..std::cmp::min(k as usize, nums.len()) - 1] {
			*cnts.entry(i).or_insert(0) += 1;
		}

		nums.iter()
			.zip(&nums[k as usize - 1..])
			.map(|(&tail, &head)| {
				*cnts.entry(head).or_insert(0) += 1;
				let v = *cnts.keys().next_back().unwrap();
				if let std::collections::btree_map::Entry::Occupied(mut e) = cnts.entry(tail) {
					*e.get_mut() -= 1;
					if *e.get() == 0 {
						e.remove_entry();
					}
				}
				v
			})
			.collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_239() {
		assert_eq!(Solution::max_sliding_window(vec![7, 2, 4], 2), [7, 4]);
		assert_eq!(
			Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
			vec![3, 3, 5, 5, 6, 7]
		);
		assert_eq!(
			Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 1),
			[1, 3, -1, -3, 5, 3, 6, 7]
		);
	}
}
