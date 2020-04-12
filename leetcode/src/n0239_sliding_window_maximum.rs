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

// 纯丢人题解, 快过7%的提交,TODO: 重写
use std::collections::BTreeMap;
impl Solution {
	pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
		let mut res = Vec::new();
		if nums.len() == 0 {
			return res;
		}
		let mut bmap = BTreeMap::new();

		for &i in &nums[..std::cmp::min(k as usize, nums.len()) - 1] {
			*bmap.entry(i).or_insert(0) += 1;
		}

		for (i, v) in nums.iter().take(nums.len() - k as usize + 1).enumerate() {
			*bmap.entry(nums[i + k as usize - 1]).or_insert(0) += 1;
			res.push(*bmap.iter().next_back().unwrap().0);

			*bmap.get_mut(v).unwrap() -= 1;
			if bmap[v] == 0 {
				bmap.remove(v);
			}
		}

		res
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_239() {
		assert_eq!(
			Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
			vec![3, 3, 5, 5, 6, 7]
		);
		assert_eq!(Solution::max_sliding_window(vec![], 0), vec![]);
	}
}
