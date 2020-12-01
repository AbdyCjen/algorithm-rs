/**
 * [309] Best Time to Buy and Sell Stock with Cooldown
 *
 * Say you have an array for which the i^th element is the price of a given stock on day i.
 *
 * Design an algorithm to find the maximum profit. You may complete as many transactions as you like (ie, buy one and sell one share of the stock multiple times) with the following restrictions:
 *
 *
 * You may not engage in multiple transactions at the same time (ie, you must sell the stock before you buy again).
 * After you sell your stock, you cannot buy stock on next day. (ie, cooldown 1 day)
 *
 *
 * Example:
 *
 *
 * Input: [1,2,3,0,2]
 * Output: 3
 * Explanation: transactions = [buy, sell, cooldown, buy, sell]
 *
 */
pub struct Solution {}

// submission codes start here

//why?....
#[allow(dead_code)]
impl Solution {
	pub fn max_profit(prices: Vec<i32>) -> i32 {
		prices
			.into_iter()
			.fold((0, std::i32::MIN, 0), |(i, j, k), val| {
				//println!("{} {} {}", i, j, k);
				(i.max(j + val), j.max(k - val), i)
			})
			.0
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_309() {
		assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
		assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1,]), 0);
		assert_eq!(Solution::max_profit(vec![1, 2, 8, 0, 2]), 7);
		assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 9]), 10);
		assert_eq!(Solution::max_profit(vec![2, 1, 4]), 3);
		assert_eq!(Solution::max_profit(vec![6, 1, 3, 2, 4, 7]), 6);
		//same peak
		assert_eq!(Solution::max_profit(vec![1, 7, 1, 1, 7]), 12);
		assert_eq!(Solution::max_profit(vec![1, 7, 1, 1, 7]), 12);
		assert_eq!(Solution::max_profit(vec![1, 6, 1, 7, 1, 1, 9]), 14);
		assert_eq!(Solution::max_profit(vec![]), 0);
	}
}
