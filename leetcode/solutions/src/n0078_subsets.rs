/**
 * [78] Subsets
 *
 * Given a set of distinct integers, nums, return all possible subsets (the power set).
 *
 * Note: The solution set must not contain duplicate subsets.
 *
 * Example:
 *
 *
 * Input: nums = [1,2,3]
 * Output:
 * [
 *   [3],
 *   [1],
 *   [2],
 *   [1,2,3],
 *   [1,3],
 *   [2,3],
 *   [1,2],
 *   []
 * ]
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
		match nums.pop() {
			Some(n) => {
				let mut ans = Self::subsets(nums);
				for i in 0..ans.len() {
					let v = ans[i].clone();
					ans[i].push(n);
					ans.push(v);
				}
				ans
			}
			None => vec![vec![]],
		}
	}
	pub fn subsets1(nums: Vec<i32>) -> Vec<Vec<i32>> {
		let mut ans = Vec::new();
		Self::solve(vec![], &nums, &mut ans);
		ans
	}
	fn solve(mut st: Vec<i32>, nums: &[i32], ans: &mut Vec<Vec<i32>>) {
		match nums {
			[] => ans.push(st),
			[n, rest @ ..] => {
				Self::solve(st.clone(), rest, ans);
				st.push(*n);
				Self::solve(st, rest, ans);
			}
		}
	}
}

// submission codes end
