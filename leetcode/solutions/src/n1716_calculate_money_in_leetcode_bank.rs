/**
 * [1716] Calculate Money in Leetcode Bank
 *
 * Hercy wants to save money for his first car. He puts money in the Leetcode bank every day.
 * He starts by putting in $1 on Monday, the first day. Every day from Tuesday to Sunday, he will put in $1 more than the day before. On every subsequent Monday, he will put in $1 more than the previous Monday.<span style="display: none;"> </span>
 * Given n, return the total amount of money he will have in the Leetcode bank at the end of the n^th day.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 4
 * Output: 10
 * Explanation: After the 4^th day, the total is 1 + 2 + 3 + 4 = 10.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 10
 * Output: 37
 * Explanation: After the 10^th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4) = 37. Notice that on the 2^nd Monday, Hercy only puts in $2.
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 20
 * Output: 96
 * Explanation: After the 20^th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4 + 5 + 6 + 7 + 8) + (3 + 4 + 5 + 6 + 7 + 8) = 96.
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
	pub fn total_money(n: i32) -> i32 {
		let mut base = 0;
		let mut ans = 0;
		for i in 0..n {
			if i % 7 == 0 {
				base += 1;
			}
			ans += base + (i % 7);
		}
		ans
	}
}

// submission codes end
