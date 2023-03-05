/**
 * [989] Add to Array-Form of Integer
 *
 * The array-form of an integer num is an array representing its digits in left to right order.
 *
 *     For example, for num = 1321, the array form is [1,3,2,1].
 *
 * Given num, the array-form of an integer, and an integer k, return the array-form of the integer num + k.
 *  
 * <strong class="example">Example 1:
 *
 * Input: num = [1,2,0,0], k = 34
 * Output: [1,2,3,4]
 * Explanation: 1200 + 34 = 1234
 *
 * <strong class="example">Example 2:
 *
 * Input: num = [2,7,4], k = 181
 * Output: [4,5,5]
 * Explanation: 274 + 181 = 455
 *
 * <strong class="example">Example 3:
 *
 * Input: num = [2,1,5], k = 806
 * Output: [1,0,2,1]
 * Explanation: 215 + 806 = 1021
 *
 *  
 * Constraints:
 *
 *     1 <= num.length <= 10^4
 *     0 <= num[i] <= 9
 *     num does not contain any leading zeros except for the zero itself.
 *     1 <= k <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn add_to_array_form(mut num: Vec<i32>, mut flow: i32) -> Vec<i32> {
		num.reverse();
		for i in &mut num {
			let sum = *i + flow;
			*i = sum % 10;
			flow = sum / 10;
		}
		while flow > 0 {
			num.push(flow % 10);
			flow /= 10;
		}
		num.reverse();
		num
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_989() {
		assert_eq!(
			Solution::add_to_array_form(vec![1, 2, 0, 0], 34),
			vec![1, 2, 3, 4]
		);
		assert_eq!(
			Solution::add_to_array_form(vec![2, 7, 4], 981),
			vec![1, 2, 5, 5]
		);
		assert_eq!(Solution::add_to_array_form(vec![1], 1000), vec![1, 0, 0, 1]);
	}
}
