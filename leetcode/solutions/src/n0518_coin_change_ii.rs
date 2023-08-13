/**
 * [518] Coin Change II
 *
 * You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
 * Return the number of combinations that make up that amount. If that amount of money cannot be made up by any combination of the coins, return 0.
 * You may assume that you have an infinite number of each kind of coin.
 * The answer is guaranteed to fit into a signed 32-bit integer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: amount = 5, coins = [1,2,5]
 * Output: 4
 * Explanation: there are four ways to make up the amount:
 * 5=5
 * 5=2+2+1
 * 5=2+1+1+1
 * 5=1+1+1+1+1
 *
 * <strong class="example">Example 2:
 *
 * Input: amount = 3, coins = [2]
 * Output: 0
 * Explanation: the amount of 3 cannot be made up just with coins of 2.
 *
 * <strong class="example">Example 3:
 *
 * Input: amount = 10, coins = [10]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= coins.length <= 300
 *     1 <= coins[i] <= 5000
 *     All the values of coins are unique.
 *     0 <= amount <= 5000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
		let mut dp = vec![0; amount as usize + 1];
		dp[0] = 1;
		for coin in coins {
			for i in 0..=amount - coin {
				dp[(i + coin) as usize] += dp[i as usize];
			}
		}
		dp[amount as usize]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_518() {
		assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
	}
}
