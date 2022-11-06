/**
 * [1359] Count All Valid Pickup and Delivery Options
 *
 * Given n orders, each order consist in pickup and delivery services.
 * Count all valid pickup/delivery possible sequences such that delivery(i) is always after of pickup(i).
 * Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 1
 * Output: 1
 * Explanation: Unique order (P1, D1), Delivery 1 always is after of Pickup 1.
 *
 * Example 2:
 *
 * Input: n = 2
 * Output: 6
 * Explanation: All possible orders:
 * (P1,P2,D1,D2), (P1,P2,D2,D1), (P1,D1,P2,D2), (P2,P1,D1,D2), (P2,P1,D2,D1) and (P2,D2,P1,D1).
 * This is an invalid order (P1,D2,P2,D1) because Pickup 2 is after of Delivery 2.
 *
 * Example 3:
 *
 * Input: n = 3
 * Output: 90
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 500
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	// 观察一个 p1, p2, ..., pn 的序列, 我们在其中插入d1, d2, ...,dn
	// 首先插入dn, 只有一个可行空缺(pn之后);
	// 接着插入d(n-1), 观察到有三个可行空缺: p(n-1)和pn之间, dn前后
	// 依次类推, 可知针对固定的pickup序列，总共有1 * 3 *..(2n - 1)个order序列
	// 根据枚举规则总共有n!个pickup序列, 所以结果为n! * 1 * 3 * ..(2n - 1)
	pub fn count_orders(n: i32) -> i32 {
		const MO: i64 = 1e9 as i64 + 7;
		(1..=n as i64)
			.map(|i| (i * 2 - 1) * i % MO)
			.fold(1, |i, v| i * v % MO) as _
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1359() {
		assert_eq!(Solution::count_orders(1), 1);
		assert_eq!(Solution::count_orders(2), 6);
		assert_eq!(Solution::count_orders(3), 90);
		assert_eq!(Solution::count_orders(8), 729647433);
	}
}
