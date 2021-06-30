/**
 * [923] 3Sum With Multiplicity
 *
 * Given an integer array arr, and an integer target, return the number of tuples i, j, k such that i < j < k and arr[i] + arr[j] + arr[k] == target.
 * As the answer can be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: arr = [1,1,2,2,3,3,4,4,5,5], target = 8
 * Output: 20
 * Explanation:
 * Enumerating by the values (arr[i], arr[j], arr[k]):
 * (1, 2, 5) occurs 8 times;
 * (1, 3, 4) occurs 8 times;
 * (2, 2, 4) occurs 2 times;
 * (2, 3, 3) occurs 2 times.
 *
 * Example 2:
 *
 * Input: arr = [1,1,2,2,2,2], target = 5
 * Output: 12
 * Explanation:
 * arr[i] = 1, arr[j] = arr[k] = 2 occurs 12 times:
 * We choose one 1 from [1,1] in 2 ways,
 * and two 2s from [2,2,2,2] in 6 ways.
 *
 *  
 * Constraints:
 *
 *     3 <= arr.length <= 3000
 *     0 <= arr[i] <= 100
 *     0 <= target <= 300
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn three_sum_multi(mut arr: Vec<i32>, target: i32) -> i32 {
		const MOD: i32 = 1_000_000_007;
		let mut ans = 0;
		arr.sort_unstable();

		for i in (2..arr.len()).rev() {
			let k = arr[i];
			let arr = &arr[..i];
			let (mut i, mut j) = (0, arr.len() - 1);
			while i < j {
				use std::cmp::Ordering;
				match (arr[i] + arr[j] + k).cmp(&target) {
					Ordering::Less => i += 1,
					Ordering::Greater => j -= 1,
					Ordering::Equal => {
						if arr[i] == arr[j] {
							let n = (j - i) as i32;
							ans = (ans + n * (n + 1) / 2) % MOD;
							i = j;
						} else {
							let (prv_i, prv_j) = (i, j);
							while arr[i] == arr[prv_i] {
								i += 1;
							}
							while arr[j] == arr[prv_j] {
								j -= 1;
							}
							ans = (ans + ((i - prv_i) * (prv_j - j)) as i32) % MOD;
						}
					}
				}
			}
		}

		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_923() {
		assert_eq!(
			Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8),
			20
		);
		assert_eq!(Solution::three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5), 12);
		assert_eq!(Solution::three_sum_multi(vec![1, 1, 1, 1, 1], 5), 0);
		assert_eq!(Solution::three_sum_multi(vec![1, 1, 1, 1, 1], 3), 10);
	}
}
