/**
 * [1207] Unique Number of Occurrences
 *
 * Given an array of integers arr, return true if the number of occurrences of each value in the array is unique, or false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [1,2,2,1,1,3]
 * Output: true
 * Explanation: The value 1 has 3 occurrences, 2 has 2 and 3 has 1. No two values have the same number of occurrences.
 * <strong class="example">Example 2:
 *
 * Input: arr = [1,2]
 * Output: false
 *
 * <strong class="example">Example 3:
 *
 * Input: arr = [-3,0,1,-3,1,1,1,-3,10,0]
 * Output: true
 *
 *  
 * Constraints:
 *
 *     1 <= arr.length <= 1000
 *     -1000 <= arr[i] <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn unique_occurrences(arr: Vec<i32>) -> bool {
		let mut cnt = std::collections::HashMap::new();
		for i in arr {
			*cnt.entry(i).or_insert(0) += 1;
		}
		let mut set = std::collections::HashSet::new();
		for c in cnt.into_values() {
			if !set.insert(c) {
				return false;
			}
		}
		true
	}
}

// submission codes end
