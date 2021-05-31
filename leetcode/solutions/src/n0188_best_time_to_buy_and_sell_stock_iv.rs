/**
 * [188] Best Time to Buy and Sell Stock IV
 *
 * Say you have an array for which the i<span style="font-size: 10.8333px;">-</span><span style="font-size: 10.8333px;">th</span> element is the price of a given stock on day i.
 * Design an algorithm to find the maximum profit. You may complete at most k transactions.
 * Note:<br />
 * You may not engage in multiple transactions at the same time (ie, you must sell the stock before you buy again).
 * Example 1:
 *
 * Input: [2,4,1], k = 2
 * Output: 2
 * Explanation: Buy on day 1 (price = 2) and sell on day 2 (price = 4), profit = 4-2 = 2.
 *
 * Example 2:
 *
 * Input: [3,2,6,5,0,3], k = 2
 * Output: 7
 * Explanation: Buy on day 2 (price = 2) and sell on day 3 (price = 6), profit = 6-2 = 4.
 *              Then buy on day 5 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
 *
 */
pub struct Solution {}

// submission codes start here

// 题解见123
#[allow(dead_code)]
impl Solution {
	pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
		if prices.is_empty() {
			return 0;
		}
		let mut cache = vec![0; prices.len()];

		for _ in 0..usize::min(k as usize, prices.len() / 2 + 1) {
			let mut max_pre_prof_price = std::i32::MIN;
			let mut max_profit = std::i32::MIN;
			for (profit, &price) in cache.iter_mut().zip(prices.iter()) {
				max_pre_prof_price = max_pre_prof_price.max(*profit - price);
				max_profit = max_profit.max(price + max_pre_prof_price);
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
	fn test_188() {
		assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
		assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
	}
}
