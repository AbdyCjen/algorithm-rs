/**
 * [1523] Count Odd Numbers in an Interval Range
 *
 * Given two non-negative integers low and <font face="monospace">high</font>. Return the count of odd numbers between low and <font face="monospace">high</font> (inclusive).
 *
 *  
 * <strong class="example">Example 1:
 *
 *
 * Input: low = 3, high = 7
 * Output: 3
 * Explanation: The odd numbers between 3 and 7 are [3,5,7].
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: low = 8, high = 10
 * Output: 1
 * Explanation: The odd numbers between 8 and 10 are [9].
 *
 *  
 * Constraints:
 *
 *
 *     0 <= low <= high <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_odds(low: i32, high: i32) -> i32 {
		if low == high {
			return (low % 2 == 1) as i32;
		}
		let l = low + (low % 2 == 0) as i32;
		let h = high - (high % 2 == 0) as i32;
		(h - l) / 2 + 1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1523() {
		assert_eq!(Solution::count_odds(3, 7), 3);
		assert_eq!(Solution::count_odds(8, 10), 1);
		assert_eq!(Solution::count_odds(8, 8), 0);
		assert_eq!(Solution::count_odds(9, 9), 1);
	}
}
