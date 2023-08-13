/**
 * [77] Combinations
 *
 * Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].
 * You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 4, k = 2
 * Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
 * Explanation: There are 4 choose 2 = 6 total combinations.
 * Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1, k = 1
 * Output: [[1]]
 * Explanation: There is 1 choose 1 = 1 total combination.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 20
 *     1 <= k <= n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		Self::solve(vec![], n, k, &mut ans);
		ans
	}
	fn solve(mut st: Vec<i32>, n: i32, k: i32, ans: &mut Vec<Vec<i32>>) {
		match k {
			0 => ans.push(st),
			_ => {
				if n > k {
					Self::solve(st.clone(), n - 1, k, ans);
				}
				st.push(n);
				Self::solve(st, n - 1, k - 1, ans)
			}
		}
	}
}

// submission codes end
