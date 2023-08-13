/**
 * [1493] Longest Subarray of 1's After Deleting One Element
 *
 * Given a binary array nums, you should delete one element from it.
 * Return the size of the longest non-empty subarray containing only 1's in the resulting array. Return 0 if there is no such subarray.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,1,0,1]
 * Output: 3
 * Explanation: After deleting the number in position 2, [1,1,1] contains 3 numbers with value of 1's.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0,1,1,1,0,1,1,0,1]
 * Output: 5
 * Explanation: After deleting the number in position 4, [0,1,1,1,1,1,0,1] longest subarray with value of 1's is [1,1,1,1,1].
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,1,1]
 * Output: 2
 * Explanation: You must delete one element.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     nums[i] is either 0 or 1.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn longest_subarray(nums: Vec<i32>) -> i32 {
		let mut left = vec![0; nums.len()];
		let mut prv = 0;
		for (cnt, &n) in left.iter_mut().zip(&nums) {
			if n == 1 {
				prv += 1;
			} else {
				prv = 0;
			}
			*cnt = prv;
		}
		prv = 0;
		let mut right = vec![0; nums.len()];
		for (cnt, &n) in right.iter_mut().zip(&nums).rev() {
			if n == 1 {
				prv += 1;
			} else {
				prv = 0;
			}
			*cnt = prv;
		}
		let mut ans = 0;
		for (i, &n) in (0..).zip(&nums) {
			if n == 1 {
				ans = ans.max(left[i]).max(right[i]);
			} else if i > 0 && i + 1 < nums.len() {
				ans = ans.max(left[i - 1] + right[i + 1]);
			}
		}
		if ans == nums.len() as i32 {
			ans - 1
		} else {
			ans
		}
	}
}

// submission codes end
