/**
 * [123] Best Time to Buy and Sell Stock III
 *
 * Say you have an array for which the i^th element is the price of a given stock on day i.
 *
 * Design an algorithm to find the maximum profit. You may complete at most two transactions.
 *
 * Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
 *
 * Example 1:
 *
 *
 * Input: [3,3,5,0,0,3,1,4]
 * Output: 6
 * Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
 *              Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
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
/*
 * 看了别人的题解, dp
 * 设f[i]表示到i之前发生一次交易能产生的最大收益, 则有:
 *  f[i] = max {
 *      f[i - 1],
 *      price[i] - min(price[j]) { j in (0..i) }
 *  }
 *  如果使用f[k,i] 表示发生在i之前发生k次交易能够得到的最大收益, 有
 *  f[k, i] = max{
 *      f[k, i - 1],
 *      price[i] + max(f[k-i, j] - price[j]) { j in (0..i)}
 *  }
*/
#[allow(dead_code)]
impl Solution {
	pub fn max_profit(prices: Vec<i32>) -> i32 {
		if prices.is_empty() {
			return 0;
		}
		let mut cache = vec![0; prices.len()];

		for _ in 0..2 {
			let mut max_pre_prof_price = i32::MIN;
			let mut max_profit = i32::MIN;
			for (profit, &price) in cache.iter_mut().zip(prices.iter()) {
				max_pre_prof_price = std::cmp::max(max_pre_prof_price, *profit - price);
				max_profit = std::cmp::max(max_profit, price + max_pre_prof_price);
				*profit = max_profit;
			}
		}

		cache[prices.len() - 1]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_123() {
		assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
		assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
		assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
		assert_eq!(Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 13);
	}
}
