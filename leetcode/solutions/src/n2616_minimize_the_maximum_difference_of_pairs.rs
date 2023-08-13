/**
 * [2616] Minimize the Maximum Difference of Pairs
 *
 * You are given a 0-indexed integer array nums and an integer p. Find p pairs of indices of nums such that the maximum difference amongst all the pairs is minimized. Also, ensure no index appears more than once amongst the p pairs.
 * Note that for a pair of elements at the index i and j, the difference of this pair is |nums[i] - nums[j]|, where |x| represents the absolute value of x.
 * Return the minimum maximum difference among all p pairs. We define the maximum of an empty set to be zero.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [10,1,2,7,1,3], p = 2
 * Output: 1
 * Explanation: The first pair is formed from the indices 1 and 4, and the second pair is formed from the indices 2 and 5.
 * The maximum difference is max(|nums[1] - nums[4]|, |nums[2] - nums[5]|) = max(0, 1) = 1. Therefore, we return 1.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [4,2,1,2], p = 1
 * Output: 0
 * Explanation: Let the indices 1 and 3 form a pair. The difference of that pair is |2 - 2| = 0, which is the minimum we can attain.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     0 <= nums[i] <= 10^9
 *     0 <= p <= (nums.length)/2
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
		nums.sort_unstable();
		let (mut l, mut r) = (0, nums.last().unwrap() - nums[0]);
		while l < r {
			let mid = (l + r) / 2;
			let (mut i, mut p) = (1, p);
			while p > 0 && i < nums.len() {
				if nums[i] - nums[i - 1] <= mid {
					p -= 1;
					i += 2;
				} else {
					i += 1;
				}
			}
			if p > 0 {
				l = mid + 1;
			} else {
				r = mid;
			}
		}
		l
	}
}

// submission codes end
