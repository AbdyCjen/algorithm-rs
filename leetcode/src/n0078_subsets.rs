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
		let mut res = Vec::new();
		Solution::subsets_rec(0, vec![], &mut res, &nums);

		res
	}

	fn subsets_rec(start: usize, mut st: Vec<i32>, res: &mut Vec<Vec<i32>>, nums: &[i32]) {
		if start >= nums.len() {
			res.push(st);
			return;
		}

		Solution::subsets_rec(start + 1, st.clone(), res, nums);
		st.push(nums[start]);
		Solution::subsets_rec(start + 1, st, res, nums);
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
