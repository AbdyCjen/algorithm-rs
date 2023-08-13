/**
 * [779] K-th Symbol in Grammar
 *
 * We build a table of n rows (1-indexed). We start by writing 0 in the 1^st row. Now in every subsequent row, we look at the previous row and replace each occurrence of 0 with 01, and each occurrence of 1 with 10.
 *
 *     For example, for n = 3, the 1^st row is 0, the 2^nd row is 01, and the 3^rd row is 0110.
 *
 * Given two integer n and k, return the k^th (1-indexed) symbol in the n^th row of a table of n rows.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 1, k = 1
 * Output: 0
 * Explanation: row 1: <u>0</u>
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 2, k = 1
 * Output: 0
 * Explanation:
 * row 1: 0
 * row 2: <u>0</u>1
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 2, k = 2
 * Output: 1
 * Explanation:
 * row 1: 0
 * row 2: 0<u>1</u>
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 30
 *     1 <= k <= 2^n - 1
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn kth_grammar(n: i32, k: i32) -> i32 {
		let mut cur = 0;
		let k = k - 1;
		let (mut l, mut r) = (0, 1 << (n - 1));
		for _ in 1..n {
			let mid = (l + r) / 2;
			if k < mid {
				r = mid;
			} else {
				cur = cur ^ 1;
				l = mid;
			}
		}
		cur
	}
}

// submission codes end
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_779() {
		assert_eq!(Solution::kth_grammar(1, 1), 0);
		assert_eq!(Solution::kth_grammar(2, 1), 0);
		assert_eq!(Solution::kth_grammar(2, 2), 1);
	}
}
