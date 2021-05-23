/**
 * [1538] Maximum Points You Can Obtain from Cards
 *
 * There are several cards arranged in a row, and each card has an associated number of points The points are given in the integer array cardPoints.
 * In one step, you can take one card from the beginning or from the end of the row. You have to take exactly k cards.
 * Your score is the sum of the points of the cards you have taken.
 * Given the integer array cardPoints and the integer k, return the maximum score you can obtain.
 *  
 * Example 1:
 *
 * Input: cardPoints = [1,2,3,4,5,6,1], k = 3
 * Output: 12
 * Explanation: After the first step, your score will always be 1. However, choosing the rightmost card first will maximize your total score. The optimal strategy is to take the three cards on the right, giving a final score of 1 + 6 + 5 = 12.
 *
 * Example 2:
 *
 * Input: cardPoints = [2,2,2], k = 2
 * Output: 4
 * Explanation: Regardless of which two cards you take, your score will always be 4.
 *
 * Example 3:
 *
 * Input: cardPoints = [9,7,7,9,7,7,9], k = 7
 * Output: 55
 * Explanation: You have to take all the cards. Your score is the sum of points of all cards.
 *
 * Example 4:
 *
 * Input: cardPoints = [1,1000,1], k = 1
 * Output: 1
 * Explanation: You cannot take the card in the middle. Your best score is 1.
 *
 * Example 5:
 *
 * Input: cardPoints = [1,79,80,1,1,1,200,1], k = 3
 * Output: 202
 *
 *  
 * Constraints:
 *
 *     1 <= cardPoints.length <= 10^5
 *     1 <= cardPoints[i] <= 10^4
 *     1 <= k <= cardPoints.length
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
		let k = k as usize;
		let tails = &card_points[card_points.len() - k..];

		let mut leftmost_sum: i32 = tails.iter().sum();
		let mut max_sum = leftmost_sum;

		// can't simply use std::iter::Iterator::scan
		// cos initial value (0 + leftmost_sum) will be skipped
		for (&i, &j) in card_points.iter().zip(tails) {
			leftmost_sum += i - j;
			max_sum = max_sum.max(leftmost_sum);
		}
		max_sum
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1538() {
		assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
		assert_eq!(Solution::max_score(vec![2, 2, 2], 2), 4);
		assert_eq!(Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
	}
}
