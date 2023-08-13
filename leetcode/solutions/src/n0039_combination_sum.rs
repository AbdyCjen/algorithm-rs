/**
 * [39] Combination Sum
 *
 * Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.
 * The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the <span data-keyword="frequency-array">frequency</span> of at least one of the chosen numbers is different.
 * The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.
 *  
 * <strong class="example">Example 1:
 *
 * Input: candidates = [2,3,6,7], target = 7
 * Output: [[2,2,3],[7]]
 * Explanation:
 * 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
 * 7 is a candidate, and 7 = 7.
 * These are the only two combinations.
 *
 * <strong class="example">Example 2:
 *
 * Input: candidates = [2,3,5], target = 8
 * Output: [[2,2,2,2],[2,3,3],[3,5]]
 *
 * <strong class="example">Example 3:
 *
 * Input: candidates = [2], target = 1
 * Output: []
 *
 *  
 * Constraints:
 *
 *     1 <= candidates.length <= 30
 *     2 <= candidates[i] <= 40
 *     All elements of candidates are distinct.
 *     1 <= target <= 40
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn combination_sum(nums: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		Self::solve(&mut vec![], &nums, n, &mut ans);
		ans
	}
	fn solve(st: &mut Vec<i32>, nums: &[i32], n: i32, ans: &mut Vec<Vec<i32>>) {
		match (n, nums) {
			(0, _) => ans.push(st.clone()),
			(_, [i, rest @ ..]) => {
				Self::solve(st, rest, n, ans);
				if *i <= n {
					st.push(*i);
					Self::solve(st, nums, n - *i, ans);
					st.pop();
				}
			}
			_ => {}
		}
	}
}

// submission codes end
