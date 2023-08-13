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

impl Solution {
	pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
		let k = k as usize;
		let mut dq = std::collections::VecDeque::new();
		for i in &nums[..k - 1] {
			while !dq.is_empty() && dq.back() < Some(i) {
				dq.pop_back();
			}
			dq.push_back(*i);
		}
		let mut ans = Vec::with_capacity(nums.len());
		for (h, t) in nums.iter().zip(&nums[k - 1..]) {
			while !dq.is_empty() && dq.back() < Some(t) {
				dq.pop_back();
			}
			dq.push_back(*t);
			ans.push(*dq.front().unwrap());
			if dq.front() == Some(h) {
				dq.pop_front();
			}
		}
		ans
	}
	pub fn max_sliding_window1(nums: Vec<i32>, k: i32) -> Vec<i32> {
		use std::collections::BTreeMap;
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
