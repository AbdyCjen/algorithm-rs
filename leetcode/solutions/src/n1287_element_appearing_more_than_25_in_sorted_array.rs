/**
 * [1287] Element Appearing More Than 25% In Sorted Array
 *
 * Given an integer array sorted in non-decreasing order, there is exactly one integer in the array that occurs more than 25% of the time, return that integer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [1,2,2,6,6,6,6,7,10]
 * Output: 6
 *
 * <strong class="example">Example 2:
 *
 * Input: arr = [1,1]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= arr.length <= 10^4
 *     0 <= arr[i] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_special_integer(arr: Vec<i32>) -> i32 {
		let mut cnts = std::collections::HashMap::new();
		let lim = arr.len() as i32 / 4;
		for i in arr {
			*cnts.entry(i).or_insert(0) += 1;
		}
		cnts.into_iter().find(|(_, c)| *c > lim).unwrap().0
	}
}

// submission codes end
