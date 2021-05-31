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

#[allow(dead_code)]
impl Solution {
	pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
		fn subsets_inner(nums: &[i32], st: Vec<i32>, ans: &mut Vec<Vec<i32>>, mut prev: i32) {
			ans.push(st.clone());

			for (i, &v) in nums.iter().enumerate() {
				if i != 0 && prev == v {
					continue;
				}
				let mut st = st.clone();
				st.push(v);
				subsets_inner(&nums[i + 1..], st, ans, v);
				prev = v;
			}
		}
		nums.sort_unstable();
		let mut ans = Vec::new();
		subsets_inner(&nums, vec![], &mut ans, -11);
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_90() {
		assert_eq!(
			Solution::subsets_with_dup(vec![1, 2, 2]),
			vec![
				vec![],
				vec![1],
				vec![1, 2],
				vec![1, 2, 2],
				vec![2],
				vec![2, 2]
			]
		);
	}
}
