/**
 * [2466] Count Ways To Build Good Strings
 *
 * Given the integers zero, one, low, and high, we can construct a string by starting with an empty string, and then at each step perform either of the following:
 *
 *     Append the character '0' zero times.
 *     Append the character '1' one times.
 *
 * This can be performed any number of times.
 * A good string is a string constructed by the above process having a length between low and high (inclusive).
 * Return the number of different good strings that can be constructed satisfying these properties. Since the answer can be large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: low = 3, high = 3, zero = 1, one = 1
 * Output: 8
 * Explanation:
 * One possible valid good string is "011".
 * It can be constructed as follows: "" -> "0" -> "01" -> "011".
 * All binary strings from "000" to "111" are good strings in this example.
 *
 * <strong class="example">Example 2:
 *
 * Input: low = 2, high = 3, zero = 1, one = 2
 * Output: 5
 * Explanation: The good strings are "00", "11", "000", "110", and "011".
 *
 *  
 * Constraints:
 *
 *     1 <= low <= high <= 10^5
 *     1 <= zero, one <= low
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
		let (zero, one) = (zero as usize, one as usize);
		let mut dp = vec![0; high as usize + 1];
		let mo = 1e9 as i32 + 7;
		dp[0] = 1;
		for i in 1..dp.len() {
			if i >= zero {
				dp[i] += dp[i - zero];
			}
			if i >= one {
				dp[i] += dp[i - one];
			}
			dp[i] %= mo;
		}
		let mut ans = 0;
		for &i in &dp[low as usize..=high as usize] {
			ans = (ans + i) % mo;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2466() {
		assert_eq!(Solution::count_good_strings(3, 3, 1, 1), 8);
		assert_eq!(Solution::count_good_strings(2, 3, 1, 2), 5);
		assert_eq!(Solution::count_good_strings(200, 200, 10, 1), 764262396);
	}
}
