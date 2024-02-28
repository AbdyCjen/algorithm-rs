/**
 * [1043] Partition Array for Maximum Sum
 *
 * Given an integer array arr, partition the array into (contiguous) subarrays of length at most k. After partitioning, each subarray has their values changed to become the maximum value of that subarray.
 * Return the largest sum of the given array after partitioning. Test cases are generated so that the answer fits in a 32-bit integer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [1,15,7,9,2,5,10], k = 3
 * Output: 84
 * Explanation: arr becomes [15,15,15,9,10,10,10]
 *
 * <strong class="example">Example 2:
 *
 * Input: arr = [1,4,1,5,7,3,6,1,9,9,3], k = 4
 * Output: 83
 *
 * <strong class="example">Example 3:
 *
 * Input: arr = [1], k = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= arr.length <= 500
 *     0 <= arr[i] <= 10^9
 *     1 <= k <= arr.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
		let mut dp = arr.clone();
		let mut ans = 0;
		for (i, &n) in (0_usize..).zip(&arr) {
			let mut ma = n;
			ans += n;
			for j in (i.saturating_sub(k as usize - 1)..i).rev() {
				ma = ma.max(arr[j]);
				if j > 0 {
					ans = ans.max((i - j + 1) as i32 * ma + dp[j - 1]);
				} else {
					ans = ma * (i - j + 1) as i32;
				}
			}
			dp[i] = ans;
		}
		ans
	}
}

// submission codes end
