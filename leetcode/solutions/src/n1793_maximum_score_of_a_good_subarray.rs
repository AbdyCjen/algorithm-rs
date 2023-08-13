/**
 * [1793] Maximum Score of a Good Subarray
 *
 * You are given an array of integers nums (0-indexed) and an integer k.
 * The score of a subarray (i, j) is defined as min(nums[i], nums[i+1], ..., nums[j]) * (j - i + 1). A good subarray is a subarray where i <= k <= j.
 * Return the maximum possible score of a good subarray.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,4,3,7,4,5], k = 3
 * Output: 15
 * Explanation: The optimal subarray is (1, 5) with a score of min(4,3,7,4,5) * (5-1+1) = 3 * 5 = 15.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [5,5,4,5,4,1,1,1], k = 0
 * Output: 20
 * Explanation: The optimal subarray is (0, 4) with a score of min(5,5,4,5,4) * (4-0+1) = 4 * 5 = 20.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 2 * 10^4
 *     0 <= k < nums.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
		let (mut l, mut r) = (k as usize, k as usize);
		let mut ans = nums[k as usize];
		let mut min = ans;
		for _ in 1..nums.len() {
			if l == 0 {
				r += 1;
				min = min.min(nums[r]);
			} else if r == nums.len() - 1 {
				l -= 1;
				min = min.min(nums[l]);
			} else if nums[l - 1] > nums[r + 1] {
				l -= 1;
				min = nums[l].min(min);
			} else {
				r += 1;
				min = nums[r].min(min);
			}
			ans = ans.max(min * (r - l + 1) as i32);
		}
		ans
	}
}

// submission codes end
