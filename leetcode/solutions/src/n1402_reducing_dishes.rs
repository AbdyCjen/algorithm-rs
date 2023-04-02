/**
 * [1402] Reducing Dishes
 *
 * A chef has collected data on the satisfaction level of his n dishes. Chef can cook any dish in 1 unit of time.
 * Like-time coefficient of a dish is defined as the time taken to cook that dish including previous dishes multiplied by its satisfaction level i.e. time[i] * satisfaction[i].
 * Return the maximum sum of like-time coefficient that the chef can obtain after dishes preparation.
 * Dishes can be prepared in any order and the chef can discard some dishes to get this maximum value.
 *  
 * <strong class="example">Example 1:
 *
 * Input: satisfaction = [-1,-8,0,5,-9]
 * Output: 14
 * Explanation: After Removing the second and last dish, the maximum total like-time coefficient will be equal to (-1*1 + 0*2 + 5*3 = 14).
 * Each dish is prepared in one unit of time.
 * <strong class="example">Example 2:
 *
 * Input: satisfaction = [4,3,2]
 * Output: 20
 * Explanation: Dishes can be prepared in any order, (2*1 + 3*2 + 4*3 = 20)
 *
 * <strong class="example">Example 3:
 *
 * Input: satisfaction = [-1,-4,-5]
 * Output: 0
 * Explanation: People do not like the dishes. No dish is prepared.
 *
 *  
 * Constraints:
 *
 *     n == satisfaction.length
 *     1 <= n <= 500
 *     -1000 <= satisfaction[i] <= 1000
 *
 */
pub struct Solution {}

impl Solution {
	pub fn max_satisfaction(mut nums: Vec<i32>) -> i32 {
		nums.sort_unstable();
		nums.into_iter()
			.rev()
			.fold((0, 0), |(s, ans), c| (s + c, ans + 0.max(s + c)))
			.1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1402() {
		assert_eq!(Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
		assert_eq!(Solution::max_satisfaction(vec![4, 3, 2]), 20);
		assert_eq!(Solution::max_satisfaction(vec![-1, -4, -5]), 0);
	}
}
