/**
 * [300] Longest Increasing Subsequence
 *
 * Given an integer array nums, return the length of the longest strictly increasing <span data-keyword="subsequence-array">subsequence</span>.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [10,9,2,5,3,7,101,18]
 * Output: 4
 * Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0,1,0,3,2,3]
 * Output: 4
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [7,7,7,7,7,7,7]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 2500
 *     -10^4 <= nums[i] <= 10^4
 *
 *  
 * Follow up: Can you come up with an algorithm that runs in O(n log(n)) time complexity?
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn length_of_lis(nums: Vec<i32>) -> i32 {
		let mut sorted = vec![];
		for i in nums {
			if let Err(pos) = sorted.binary_search(&i) {
				if let Some(v) = sorted.get_mut(pos) {
					*v = i;
				} else {
					sorted.push(i);
				}
			}
		}
		sorted.len() as _
	}
	pub fn length_of_lis1(nums: Vec<i32>) -> i32 {
		let mut ans = vec![];
		for &i in &nums {
			ans.push(
				nums.iter()
					.zip(&ans)
					.filter(|(ii, _)| **ii < i)
					.map(|(_, a)| *a)
					.max()
					.unwrap_or(0) + 1,
			)
		}
		ans.into_iter().max().unwrap()
	}
}

// submission codes end
