/**
 * [46] Permutations
 *
 * Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 * <strong class="example">Example 2:
 * Input: nums = [0,1]
 * Output: [[0,1],[1,0]]
 * <strong class="example">Example 3:
 * Input: nums = [1]
 * Output: [[1]]
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 6
 *     -10 <= nums[i] <= 10
 *     All the integers of nums are unique.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		Self::solve(nums, 0, &mut ans);
		ans
	}

	fn solve(mut nums: Vec<i32>, i: usize, ans: &mut Vec<Vec<i32>>) {
		if nums[i..].is_empty() {
			ans.push(nums);
		} else {
			for j in i..nums.len() {
				nums.swap(i, j);
				Self::solve(nums.clone(), i + 1, ans);
			}
		}
	}
}

// submission codes end
