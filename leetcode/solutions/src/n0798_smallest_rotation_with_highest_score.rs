/**
 * [798] Smallest Rotation with Highest Score
 *
 * Given an array nums, we may rotate it by a non-negative integer k so that the array becomes nums[k], nums[k+1], nums[k+2], ... nums[nums.length - 1], nums[0], nums[1], ..., nums[k-1].  Afterward, any entries that are less than or equal to their index are worth 1 point.
 * For example, if we have [2, 4, 1, 3, 0], and we rotate by k = 2, it becomes [1, 3, 0, 2, 4]. This is worth 3 points because 1 > 0 [no points], 3 > 1 [no points], 0 <= 2 [one point], 2 <= 3 [one point], 4 <= 4 [one point].
 * Over all possible rotations, return the rotation index k that corresponds to the highest score we could receive. If there are multiple answers, return the smallest such index k.
 *
 * Example 1:
 * Input: [2, 3, 1, 4, 0]
 * Output: 3
 * Explanation:  
 * Scores for each k are listed below:
 * k = 0,  nums = [2,3,1,4,0],    score 2
 * k = 1,  nums = [3,1,4,0,2],    score 3
 * k = 2,  nums = [1,4,0,2,3],    score 3
 * k = 3,  nums = [4,0,2,3,1],    score 4
 * k = 4,  nums = [0,2,3,1,4],    score 3
 *
 * So we should choose k = 3, which has the highest score.
 *  
 *
 * Example 2:
 * Input: [1, 3, 0, 2, 4]
 * Output: 0
 * Explanation: nums will always have 3 points no matter how it shifts.
 * So we will choose the smallest k, which is 0.
 *
 * Note:
 *
 *     nums will have length at most 20000.
 *     nums[i] will be in the range [0, nums.length].
 *
 */
pub struct Solution {}

// submission codes start here

/*
 * after rotate-k, the position of element in array:
 *	i in [0..k) => ai -> i + length - k
 *	i in [k..length) ai -> i - k
 * 扫描两次数组:
 * 一次从后往前, for k in (0..n).rev()
 *	=> 如果当前a_k 在k变换后能得分, 则对于k' in [0..k], a_k 都能得分;
 *	=> 如果当前a_k 在k变换后不能得分, 但是对于k' in [0..k - a_k], a_k都能得分;
 * 一次从前往后, for k in (1..n)
 *  => 如果当前a_(k-1) 在 k变换后能得分, 那么针对 k' in [k, length + k - a_(k-1)], a_(k-1)
 *  都能得分;
 *  => 否则a_(k-1) 在k' in [k..) 都无法得分
 */
#[allow(dead_code)]
impl Solution {
	pub fn best_rotation(nums: Vec<i32>) -> i32 {
		let n = nums.len();
		let mut scores = vec![0; n];
		let mut prev = 0;
		for (k, &ai) in (1..n).zip(nums.iter()) {
			scores[k] += prev;
			if ai < n as i32 {
				scores[k] += 1;
				if let Some(fut_scr) = (k + n)
					.checked_sub(ai as usize)
					.map(|j| scores.get_mut(j))
					.flatten()
				{
					*fut_scr -= 1;
				}
			}
			prev = scores[k];
		}

		let mut scores_rev = vec![0; n];
		prev = 0;
		for (k, &ai) in nums.iter().enumerate().rev() {
			scores_rev[k] += prev;
			if ai == 0 {
				scores_rev[k] += 1;
			} else if let Some(fut_scr) = k
				.checked_sub(ai as usize)
				.map(|j| scores_rev.get_mut(j))
				.flatten()
			{
				*fut_scr += 1;
			}
			prev = scores_rev[k];
		}
		scores
			.into_iter()
			.zip(scores_rev)
			.enumerate()
			.rev()
			.max_by_key(|(_, (i, j))| i + j)
			.unwrap()
			.0 as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_798() {
		assert_eq!(Solution::best_rotation(vec![2, 3, 1, 4, 0]), 3);
		assert_eq!(Solution::best_rotation(vec![1, 3, 0, 2, 4]), 0);
	}
}
