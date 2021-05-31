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

#[allow(dead_code)]
impl Solution {
	pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
		fn subsets_inner(nums: &[i32], mut st: Vec<i32>, ans: &mut Vec<Vec<i32>>) {
			if nums.is_empty() {
				ans.push(st);
				return;
			}

			subsets_inner(&nums[1..], st.clone(), ans);
			st.push(nums[0]);
			subsets_inner(&nums[1..], st, ans);
		}
		let mut ans = Vec::new();
		subsets_inner(&nums, vec![], &mut ans);
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_78() {
		assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
		assert_eq!(Solution::subsets(vec![1]), vec![vec![], vec![1]]);
		assert_eq!(
			Solution::subsets(vec![1, 2]),
			vec![vec![], vec![2], vec![1], vec![1, 2]]
		);
	}
}
