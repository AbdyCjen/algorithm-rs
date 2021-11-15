/**
 * [629] K Inverse Pairs Array
 *
 * For an integer array nums, an inverse pair is a pair of integers [i, j] where 0 <= i < j < nums.length and nums[i] > nums[j].
 * Given two integers n and k, return the number of different arrays consist of numbers from 1 to n such that there are exactly k inverse pairs. Since the answer can be huge, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 3, k = 0
 * Output: 1
 * Explanation: Only the array [1,2,3] which consists of numbers from 1 to 3 has exactly 0 inverse pairs.
 *
 * Example 2:
 *
 * Input: n = 3, k = 1
 * Output: 2
 * Explanation: The array [1,3,2] and [2,1,3] have exactly 1 inverse pair.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 1000
 *     0 <= k <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
		let (n, k) = (n as usize, k as usize);
		return k_inverse_pairs_inner(n, k, &mut vec![vec![-1; k + 1]; n + 1]);

		fn k_inverse_pairs_inner(n: usize, k: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
			if n == 0 {
				return 0;
			} else if k == 0 {
				return 1;
			} else if cache[n][k] >= 0 {
				return cache[n][k];
			}
			cache[n][k] = (0..n.min(k + 1)).fold(0, |acc, i| {
				(acc + k_inverse_pairs_inner(n - 1, k - i, cache)) % (1e9 + 7.0) as i32
			});
			cache[n][k]
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_629() {
		assert_eq!(Solution::k_inverse_pairs(3, 0), 1);
		assert_eq!(Solution::k_inverse_pairs(3, 1), 2);
		assert_eq!(Solution::k_inverse_pairs(1000, 1000), 663677020);
	}
}
