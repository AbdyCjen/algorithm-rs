/**
 * [233] Number of Digit One
 *
 * Given an integer n, count the total number of digit 1 appearing in all non-negative integers less than or equal to n.
 *  
 * Example 1:
 *
 * Input: n = 13
 * Output: 6
 *
 * Example 2:
 *
 * Input: n = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     0 <= n <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn count_digit_one(n: i32) -> i32 {
		let mut ans = 0;

		let n = n as i64;
		let mut i = 1;
		while i <= n {
			let (h, l) = (n / (i * 10), n % (i * 10));
			ans += h * i // 0..h*i*10 i位1
				+ (l - i + 1).clamp(0, i); // (h*i*10)..n i位1 => 等价于(0..n % (i * 10)) i位1
						   // => i 为10的幂, 观察可知, (0..i * 10)中i位1都集中在(i..2*i)区间
			i *= 10;
		}

		ans as i32
	}
	pub fn count_digit_one_01(n: i32) -> i32 {
		// 将n拆成0..(n/10*10)和(n/ 10 * 10)..=n 两部分分别计算个位和高位1次数
		match n {
			0 => 0,
			1..=9 => 1,
			n => {
				let (h, l) = (n / 10, n % 10);
				Self::count_digit_one_01(h - 1) * 10 // 0..(h * 10) 高位1
					+ h // 0..(h * 10) 个位1
					+ Self::count_one(h) * (l + 1) // (h * 10)..=(h * 10 + l) 高位1
					+ (l >= 1) as i32 // (h * 10)..=(h * 10 + l) 个位1
			}
		}
	}
	fn count_one(mut n: i32) -> i32 {
		let mut ans = 0;
		while n > 0 {
			ans += (n % 10 == 1) as i32;
			n /= 10;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_233() {
		assert_eq!(Solution::count_digit_one(1e5 as i32), 50001);
		assert_eq!(Solution::count_digit_one(999999999), 900000000);
		assert_eq!(Solution::count_digit_one(13), 6);
		assert_eq!(Solution::count_digit_one(0), 0);
		assert_eq!(Solution::count_digit_one(1e9 as i32 - 1), 900000000);
	}
}
