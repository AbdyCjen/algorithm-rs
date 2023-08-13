/**
 * [322] Coin Change
 *
 * You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
 * Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
 * You may assume that you have an infinite number of each kind of coin.
 *  
 * <strong class="example">Example 1:
 *
 * Input: coins = [1,2,5], amount = 11
 * Output: 3
 * Explanation: 11 = 5 + 5 + 1
 *
 * <strong class="example">Example 2:
 *
 * Input: coins = [2], amount = 3
 * Output: -1
 *
 * <strong class="example">Example 3:
 *
 * Input: coins = [1], amount = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     1 <= coins.length <= 12
 *     1 <= coins[i] <= 2^31 - 1
 *     0 <= amount <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
		let mut dp = vec![None; amount as usize + 1];
		dp[0] = Some(0);
		for coin in coins {
			for i in 0..=amount - coin {
				if let Some(n) = dp[i as usize] {
					let next = &mut dp[(i + coin) as usize];
					*next = Some((n + 1).min((*next).unwrap_or(i32::MAX)));
				}
			}
		}
		dp[amount as usize].unwrap()
	}
}

// submission codes end
