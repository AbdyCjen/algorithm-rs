/**
 * [441] Arranging Coins
 *
 * You have a total of n coins that you want to form in a staircase shape, where every k-th row must have exactly k coins.
 *  
 * Given n, find the total number of full staircase rows that can be formed.
 *
 * n is a non-negative integer and fits within the range of a 32-bit signed integer.
 *
 * Example 1:
 *
 * n = 5
 *
 * The coins can form the following rows:
 * ¤
 * ¤ ¤
 * ¤ ¤
 *
 * Because the 3rd row is incomplete, we return 2.
 *
 *
 *
 * Example 2:
 *
 * n = 8
 *
 * The coins can form the following rows:
 * ¤
 * ¤ ¤
 * ¤ ¤ ¤
 * ¤ ¤
 *
 * Because the 4th row is incomplete, we return 3.
 *
 *
 */
#[allow(dead_code)]
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn arrange_coins(n: i32) -> i32 {
		(0_i64..)
			.find(|i| (i + 2) * (i + 1) > 2 * n as i64)
			.unwrap_or(0) as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_441() {
		assert_eq!(Solution::arrange_coins(8), 3);
		assert_eq!(Solution::arrange_coins(5), 2);
		assert_eq!(Solution::arrange_coins(0), 0);
		assert_eq!(Solution::arrange_coins(1), 1);
	}
}
