/**
 * [790] Domino and Tromino Tiling
 *
 * You have two types of tiles: a 2 x 1 domino shape and a tromino shape. You may rotate these shapes.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/15/lc-domino.jpg" style="width: 362px; height: 195px;" />
 * Given an integer n, return the number of ways to tile an 2 x n board. Since the answer may be very large, return it modulo 10^9 + 7.
 * In a tiling, every square must be covered by a tile. Two tilings are different if and only if there are two 4-directionally adjacent cells on the board such that exactly one of the tilings has both squares occupied by a tile.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/15/lc-domino1.jpg" style="width: 500px; height: 226px;" />
 * Input: n = 3
 * Output: 5
 * Explanation: The five different ways are show above.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn num_tilings(n: i32) -> i32 {
		let (mut whole, mut half) = (vec![1; n as usize + 1], vec![0; n as usize + 1]);
		const MO: i32 = (1e9 + 7.0) as i32;
		for i in 2..=(n as usize) {
			whole[i] = (whole[i - 1] + whole[i - 2]) % MO; // last piece is vertical domino + last two pieces is two horizontal domino
			whole[i] = (whole[i] + half[i - 2] * 2 % MO) % MO; // last piece is a tromino (with two different direction)
			half[i - 1] = (half[i - 2] + whole[i - 2]) % MO;
		}
		whole[n as usize]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_790() {
		assert_eq!(Solution::num_tilings(1), 1);
		assert_eq!(Solution::num_tilings(2), 2);
		assert_eq!(Solution::num_tilings(3), 5);
		assert_eq!(Solution::num_tilings(30), 312342182);
	}
}
