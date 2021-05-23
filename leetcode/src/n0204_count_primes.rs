/**
 * [204] Count Primes
 *
 * Count the number of prime numbers less than a non-negative number, n.
 *  
 * Example 1:
 *
 * Input: n = 10
 * Output: 4
 * Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
 *
 * Example 2:
 *
 * Input: n = 0
 * Output: 0
 *
 * Example 3:
 *
 * Input: n = 1
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     0 <= n <= 5 * 10^6
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn count_primes(n: i32) -> i32 {
		if n < 2 {
			return 0;
		}
		let n = n as usize;
		let mut bmap = vec![true; n];
		bmap[0] = false;
		bmap[1] = false;
		for i in (4..n).step_by(2) {
			bmap[i] = false;
		}

		for i in (3..=((n as f32).sqrt() as usize)).step_by(2) {
			if bmap[i] {
				for j in (i * i..n).step_by(i * 2) {
					bmap[j] = false;
				}
			}
		}

		bmap.into_iter().filter(|&i| i).count() as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_204() {
		assert_eq!(Solution::count_primes(10), 4);
		assert_eq!(Solution::count_primes(17749), 2037);
		assert_eq!(Solution::count_primes(9), 4);
	}
}
