/**
 * [1155] Number of Dice Rolls With Target Sum
 *
 * You have n dice, and each die has k faces numbered from 1 to k.
 * Given three integers n, k, and target, return the number of possible ways (out of the k^n total ways) to roll the dice, so the sum of the face-up numbers equals target. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 1, k = 6, target = 3
 * Output: 1
 * Explanation: You throw one die with 6 faces.
 * There is only one way to get a sum of 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 2, k = 6, target = 7
 * Output: 6
 * Explanation: You throw two dice, each with 6 faces.
 * There are 6 ways to get a sum of 7: 1+6, 2+5, 3+4, 4+3, 5+2, 6+1.
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 30, k = 30, target = 500
 * Output: 222616187
 * Explanation: The answer must be returned modulo 10^9 + 7.
 *
 *  
 * Constraints:
 *
 *     1 <= n, k <= 30
 *     1 <= target <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
		let mut dp = std::collections::BTreeMap::new();
		dp.insert(0, 1);
		for _ in 0..n {
			for (n, c) in std::mem::take(&mut dp) {
				for i in n + 1..=(n + k).min(target) {
					let e = dp.entry(k + i).or_insert(0);
					*e = (*e + c) % (1e9 as i32 + 7);
				}
			}
		}
		dp.remove(&target).unwrap_or(0)
	}
}

// submission codes end
