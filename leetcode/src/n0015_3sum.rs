/**
 * [15] 3Sum
 *
 * Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.
 *
 * Note:
 *
 * The solution set must not contain duplicate triplets.
 *
 * Example:
 *
 *
 * Given array nums = [-1, 0, 1, 2, -1, -4],
 *
 * A solution set is:
 * [
 *   [-1, 0, 1],
 *   [-1, -1, 2]
 * ]
 *
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
		let mut res = Vec::new();
		if nums.len() < 3 {
			return res;
		}
		nums.sort_unstable();
		for i in (2..nums.len()).rev() {
			if nums.get(i) == nums.get(i + 1) {
				continue;
			}
			let v = nums[i];
			let nums = &nums[..i];
			let (mut lb, mut ub) = (0, nums.len() - 1);
			while lb < ub {
				match (nums[lb] + nums[ub] + v).cmp(&0) {
					std::cmp::Ordering::Less => lb += 1,
					std::cmp::Ordering::Greater => ub -= 1,
					std::cmp::Ordering::Equal => {
						res.push(vec![nums[lb], nums[ub], v]);
						lb += 1;
						ub -= 1;
						while lb < ub && nums[lb] == nums[lb - 1] {
							lb += 1;
						}
					}
				};
			}
		}
		res
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_15() {
		assert_eq!(
			Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
			vec![vec![-1, -1, 2], vec![-1, 0, 1],]
		);
		assert_eq!(Solution::three_sum(vec![0, 0]), Vec::<Vec<i32>>::new());
		assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
	}
}
