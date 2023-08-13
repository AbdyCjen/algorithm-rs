/**
 * [90] Subsets II
 *
 * Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
 * The solution set must not contain duplicate subsets. Return the solution in any order.
 *  
 * Example 1:
 * Input: nums = [1,2,2]
 * Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
 * Example 2:
 * Input: nums = [0]
 * Output: [[],[0]]
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10
 *     -10 <= nums[i] <= 10
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
		let mut ans = Vec::new();
		nums.sort_unstable();
		Self::solve(vec![], &nums, &mut ans, i32::MAX);
		ans
	}
	fn solve(mut st: Vec<i32>, nums: &[i32], ans: &mut Vec<Vec<i32>>, prv: i32) {
		match nums {
			[] => ans.push(st),
			[n, rest @ ..] => {
				if prv != *n {
					Self::solve(st.clone(), rest, ans, prv);
				}
				st.push(*n);
				Self::solve(st, rest, ans, *n);
			}
		}
	}
}

// submission codes end
