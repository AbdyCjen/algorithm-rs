/**
 * [1420] Build Array Where You Can Find The Maximum Exactly K Comparisons
 *
 * You are given three integers n, m and k. Consider the following algorithm to find the maximum element of an array of positive integers:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/02/e.png" style="width: 424px; height: 372px;" />
 * You should build the array arr which has the following properties:
 *
 *     arr has exactly n integers.
 *     1 <= arr[i] <= m where (0 <= i < n).
 *     After applying the mentioned algorithm to arr, the value search_cost is equal to k.
 *
 * Return the number of ways to build the array arr under the mentioned conditions. As the answer may grow large, the answer must be computed modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 2, m = 3, k = 1
 * Output: 6
 * Explanation: The possible arrays are [1, 1], [2, 1], [2, 2], [3, 1], [3, 2] [3, 3]
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 5, m = 2, k = 3
 * Output: 0
 * Explanation: There are no possible arrays that satisify the mentioned conditions.
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 9, m = 1, k = 1
 * Output: 1
 * Explanation: The only possible array is [1, 1, 1, 1, 1, 1, 1, 1, 1]
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 50
 *     1 <= m <= 100
 *     0 <= k <= n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
		let mut mat = vec![vec![vec![0; 1 + k as usize]; 1 + m as usize]; 1 + n as usize];
		const MO: u64 = 1e9 as u64 + 7;
		for j in 1..=m {
			mat[1][j as usize][1] = 1;
		}
		for a in 1..=n as usize {
			for b in 1..=m as usize {
				for c in 1..=k as usize {
					let mut s = (b as u64 * mat[a - 1][b][c]) % MO;
					for x in 1..b {
						s = (s + mat[a - 1][x][c - 1]) % MO;
					}
					mat[a][b][c] = (mat[a][b][c] + s) % MO;
				}
			}
		}
		(1..=m).fold(0, |a, j| {
			(a + mat[n as usize][j as usize][k as usize] as i32) % MO as i32
		})
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1420() {
		assert_eq!(Solution::num_of_arrays(5, 2, 3), 0);
		assert_eq!(Solution::num_of_arrays(9, 1, 1), 1);
		assert_eq!(Solution::num_of_arrays(2, 3, 1), 6);
	}
}
