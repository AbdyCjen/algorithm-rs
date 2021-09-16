/**
 * [89] Gray Code
 *
 * An n-bit gray code sequence is a sequence of 2^n integers where:
 *
 *     Every integer is in the inclusive range [0, 2^n - 1],
 *     The first integer is 0,
 *     An integer appears no more than once in the sequence,
 *     The binary representation of every pair of adjacent integers differs by exactly one bit, and
 *     The binary representation of the first and last integers differs by exactly one bit.
 *
 * Given an integer n, return any valid n-bit gray code sequence.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: [0,1,3,2]
 * Explanation:
 * The binary representation of [0,1,3,2] is [00,01,11,10].
 * - 0<u>0</u> and 0<u>1</u> differ by one bit
 * - <u>0</u>1 and <u>1</u>1 differ by one bit
 * - 1<u>1</u> and 1<u>0</u> differ by one bit
 * - <u>1</u>0 and <u>0</u>0 differ by one bit
 * [0,2,3,1] is also a valid gray code sequence, whose binary representation is [00,10,11,01].
 * - <u>0</u>0 and <u>1</u>0 differ by one bit
 * - 1<u>0</u> and 1<u>1</u> differ by one bit
 * - <u>1</u>1 and <u>0</u>1 differ by one bit
 * - 0<u>1</u> and 0<u>0</u> differ by one bit
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: [0,1]
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 16
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn gray_code(n: i32) -> Vec<i32> {
		if n == 1 {
			return vec![0, 1];
		}
		let mut ans = Self::gray_code(n - 1);
		for i in (0..ans.len()).rev() {
			ans.push(ans[i] + (1 << (n - 1)));
		}

		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_89() {
		fn check_gray(codec: &[i32]) -> bool {
			let mut ans = true;
			let mut wit = codec.windows(2);
			while let Some(&[a, b]) = wit.next() {
				ans &= (a ^ b).count_ones() == 1;
			}
			ans
		}

		assert!(check_gray(&Solution::gray_code(1)));
		assert!(check_gray(&Solution::gray_code(2)));
		assert!(check_gray(&Solution::gray_code(3)));
		assert!(check_gray(&Solution::gray_code(4)));
		assert!(check_gray(&Solution::gray_code(5)));
		assert!(check_gray(&Solution::gray_code(6)));
		assert!(check_gray(&Solution::gray_code(7)));
	}
}
