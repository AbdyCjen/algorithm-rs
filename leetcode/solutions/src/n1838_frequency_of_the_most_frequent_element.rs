/**
 * [1838] Frequency of the Most Frequent Element
 *
 * The frequency of an element is the number of times it occurs in an array.
 * You are given an integer array nums and an integer k. In one operation, you can choose an index of nums and increment the element at that index by 1.
 * Return the maximum possible frequency of an element after performing at most k operations.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,4], k = 5
 * Output: 3
 * Explanation: Increment the first element three times and the second element two times to make nums = [4,4,4].
 * 4 has a frequency of 3.
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,4,8,13], k = 5
 * Output: 2
 * Explanation: There are multiple optimal solutions:
 * - Increment the first element three times to make nums = [4,4,8,13]. 4 has a frequency of 2.
 * - Increment the second element four times to make nums = [1,8,8,13]. 8 has a frequency of 2.
 * - Increment the third element five times to make nums = [1,4,13,13]. 13 has a frequency of 2.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [3,9,6], k = 2
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^5
 *     1 <= k <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
		nums.sort_unstable();
		let mut i = 0;
		let mut ans = 1;
		let mut s = 0;
		for (j, &n) in (0..).zip(&nums) {
			s += n;
			while n * (j - i + 1) as i32 - s > k {
				s -= nums[i];
				i += 1;
			}
			ans = ans.max(j - i + 1);
		}
		ans as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1838() {
		assert_eq!(Solution::max_frequency(vec![3, 9, 6], 2), 1);
	}
}
