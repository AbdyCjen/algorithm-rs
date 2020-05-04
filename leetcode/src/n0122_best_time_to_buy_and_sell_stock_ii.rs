/**
 * [122] Best Time to Buy and Sell Stock II
 *
 * Say you have an array for which the i^th element is the price of a given stock on day i.
 *
 * Design an algorithm to find the maximum profit. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).
 *
 * Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
 *
 * Example 1:
 *
 *
 * Input: [7,1,5,3,6,4]
 * Output: 7
 * Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
 *              Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
 *
 *
 * Example 2:
 *
 *
 * Input: [1,2,3,4,5]
 * Output: 4
 * Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
 *              Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
 *              engaging multiple transactions at the same time. You must sell before buying again.
 *
 *
 * Example 3:
 *
 *
 * Input: [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transaction is done, i.e. max profit = 0.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn max_profit(prices: Vec<i32>) -> i32 {
		let mut pro_sum = 0;
		let mut cur_pro = 0;
		let mut cur_lo = std::i32::MAX - 1;
		for &price in prices.iter() {
			if price - cur_lo > cur_pro {
				cur_pro = price - cur_lo;
			} else {
				pro_sum += cur_pro;
				cur_pro = 0;
				cur_lo = price;
			}
		}
		pro_sum + cur_pro
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_122() {
		assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
		assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
		assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
	}
}
