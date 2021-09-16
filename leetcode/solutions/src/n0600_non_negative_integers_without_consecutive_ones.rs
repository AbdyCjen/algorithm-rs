/**
 * [600] Non-negative Integers without Consecutive Ones
 *
 * Given a positive integer n, return the number of the integers in the range [0, n] whose binary representations do not contain consecutive ones.
 *  
 * Example 1:
 *
 * Input: n = 5
 * Output: 5
 * Explanation:
 * Here are the non-negative integers <= 5 with their corresponding binary representations:
 * 0 : 0
 * 1 : 1
 * 2 : 10
 * 3 : 11
 * 4 : 100
 *
 * 5 : 101
 * Among them, only integer 3 disobeys the rule (two consecutive ones) and the other 5 satisfy the rule.
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 2
 *
 * Example 3:
 *
 * Input: n = 2
 * Output: 3
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn find_integers(n: i32) -> i32 {
		// dp[i] => 小于2 ^ i且满足条件的整数数量
		let mut dp = [0; 32];
		dp[0] = 1;
		let mut pp = 1;
		for i in (1..32).take_while(|&i| 1 << i <= n) {
			dp[i] = pp + dp[i - 1];
			pp = dp[i - 1];
		}

		let mut ans = 1;
		for (p, v) in (0..31).map(|i| 1 << i).zip(&dp).rev() {
			if p & n > 0 {
				ans += v;
				// 对于形如a = 0b11xxxx的数, 只要计算
				// dp[k]+dp[k-1](k=a.next_power_of_two()-1) 就行
				if (p << 1) & n > 0 {
					ans -= 1;
					break;
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
	fn test_600() {
		assert_eq!(Solution::find_integers(1), 2);
		assert_eq!(Solution::find_integers(2), 3);
		assert_eq!(Solution::find_integers(4), 4);
		assert_eq!(Solution::find_integers(5), 5);
		assert_eq!(Solution::find_integers(6), 5);
		assert_eq!(
			Solution::find_integers(0b1101),
			Solution::find_integers(0b1010)
		);
	}
}
