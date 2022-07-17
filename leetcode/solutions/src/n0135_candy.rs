/**
 * [135] Candy
 *
 * There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.
 * You are giving candies to these children subjected to the following requirements:
 *
 *     Each child must have at least one candy.
 *     Children with a higher rating get more candies than their neighbors.
 *
 * Return the minimum number of candies you need to have to distribute the candies to the children.
 *  
 * Example 1:
 *
 * Input: ratings = [1,0,2]
 * Output: 5
 * Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
 *
 * Example 2:
 *
 * Input: ratings = [1,2,2]
 * Output: 4
 * Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
 * The third child gets 1 candy because it satisfies the above two conditions.
 *
 *  
 * Constraints:
 *
 *     n == ratings.length
 *     1 <= n <= 2 * 10^4
 *     0 <= ratings[i] <= 2 * 10^4
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn candy(ratings: Vec<i32>) -> i32 {
		let mut prv = ratings[0];
		let mut cur = 1;
		let mut candies = vec![0; ratings.len()];

		for (candy, &i) in candies.iter_mut().zip(&ratings) {
			cur = if prv < i { cur + 1 } else { 1 };
			*candy = cur;
			prv = i;
		}

		for (candy, &i) in candies.iter_mut().zip(&ratings).rev() {
			cur = if prv < i { cur + 1 } else { 1 };
			*candy = cur.max(*candy);
			prv = i;
		}
		candies.into_iter().sum()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_135() {
		assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
		assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
		assert_eq!(Solution::candy(vec![2, 3, 4, 3, 2, 1]), 13);
	}
}
