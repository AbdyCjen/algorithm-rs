/**
 * [3101] Count Alternating Subarrays
 *
 * You are given a <span data-keyword="binary-array">binary array</span> nums.
 * We call a <span data-keyword="subarray-nonempty">subarray</span> alternating if no two adjacent elements in the subarray have the same value.
 * Return the number of alternating subarrays in nums.
 *  
 * <strong class="example">Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [0,1,1,1]</span>
 * Output: <span class="example-io">5</span>
 * Explanation:
 * The following subarrays are alternating: [0], [1], [1], [1], and [0,1].
 * </div>
 * <strong class="example">Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [1,0,1,0]</span>
 * Output: <span class="example-io">10</span>
 * Explanation:
 * Every subarray of the array is alternating. There are 10 possible subarrays that we can choose.
 * </div>
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
	pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
		let mut i = 0;
		(1..).zip(nums.windows(2)).fold(0, |acc, (j, w)| {
			if w[0] == w[1] {
				i = j;
			}
			acc + j - i + 1
		}) + 1
	}
}

// submission codes end
