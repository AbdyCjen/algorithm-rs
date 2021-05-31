/**
 * [377] Combination Sum IV
 *
 * Given an array of distinct integers nums and a target integer target, return the number of possible combinations that add up to target.
 * The answer is guaranteed to fit in a 32-bit integer.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3], target = 4
 * Output: 7
 * Explanation:
 * The possible combination ways are:
 * (1, 1, 1, 1)
 * (1, 1, 2)
 * (1, 2, 1)
 * (1, 3)
 * (2, 1, 1)
 * (2, 2)
 * (3, 1)
 * Note that different sequences are counted as different combinations.
 *
 * Example 2:
 *
 * Input: nums = [9], target = 3
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 200
 *     1 <= nums[i] <= 1000
 *     All the elements of nums are unique.
 *     1 <= target <= 1000
 *
 *  
 * Follow up: What if negative numbers are allowed in the given array? How does it change the problem? What limitation we need to add to the question to allow negative numbers?
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
		fn combination_sum4_inner(nums: &[i32], target: i32, cache: &mut [i32]) -> i32 {
			match target.cmp(&0) {
				std::cmp::Ordering::Equal => return 1,
				std::cmp::Ordering::Less => return 0,
				_ => {}
			};
			let target_usize = target as usize;
			if cache[target_usize] >= 0 {
				return cache[target_usize];
			}

			let comb_cnt: i32 = nums
				.iter()
				.map(|&n| combination_sum4_inner(nums, target - n, cache))
				.sum();

			cache[target_usize] = comb_cnt;
			comb_cnt
		}

		let mut cache = vec![-1; target as usize + 1];
		combination_sum4_inner(&nums, target, &mut cache)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_377() {
		assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
		assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
		assert_eq!(
			Solution::combination_sum4(
				vec![
					10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170,
					180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300, 310, 320, 330,
					340, 350, 360, 370, 380, 390, 400, 410, 420, 430, 440, 450, 460, 470, 480, 490,
					500, 510, 520, 530, 540, 550, 560, 570, 580, 590, 600, 610, 620, 630, 640, 650,
					660, 670, 680, 690, 700, 710, 720, 730, 740, 750, 760, 770, 780, 790, 800, 810,
					820, 830, 840, 850, 860, 870, 880, 890, 900, 910, 920, 930, 940, 950, 960, 970,
					980, 990, 111
				],
				999
			),
			1
		);
	}
}
