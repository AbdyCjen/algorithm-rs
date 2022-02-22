/**
 * [397] Integer Replacement
 *
 * Given a positive integer n, you can apply one of the following operations:
 * <ol>
 *     If n is even, replace n with n / 2.
 *     If n is odd, replace n with either n + 1 or n - 1.
 * </ol>
 * Return the minimum number of operations needed for n to become 1.
 *  
 * Example 1:
 *
 * Input: n = 8
 * Output: 3
 * Explanation: 8 -> 4 -> 2 -> 1
 *
 * Example 2:
 *
 * Input: n = 7
 * Output: 4
 * Explanation: 7 -> 8 -> 4 -> 2 -> 1
 * or 7 -> 6 -> 3 -> 2 -> 1
 *
 * Example 3:
 *
 * Input: n = 4
 * Output: 2
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn integer_replacement(n: i32) -> i32 {
		use std::collections::HashMap;
		return int_repl(n, &mut HashMap::new());
		fn int_repl(mut n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
			if let Some(&i) = cache.get(&n) {
				return i;
			}
			let mut ans = n.trailing_zeros() as i32;
			n >>= ans;

			if n != 1 {
				ans += 2 + int_repl(n / 2 + 1, cache).min(int_repl(n / 2, cache));

				cache.insert(n, ans);
			}
			ans
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_397() {
		assert_eq!(Solution::integer_replacement(8), 3);
		assert_eq!(Solution::integer_replacement(9), 4);
		assert_eq!(Solution::integer_replacement(7), 4);
		assert_eq!(Solution::integer_replacement(4), 2);
		assert_eq!(Solution::integer_replacement(2), 1);
		assert_eq!(Solution::integer_replacement(1), 0);
		assert_eq!(Solution::integer_replacement(1073741823), 31);
		assert_eq!(Solution::integer_replacement(2147483647), 32);
	}
}
