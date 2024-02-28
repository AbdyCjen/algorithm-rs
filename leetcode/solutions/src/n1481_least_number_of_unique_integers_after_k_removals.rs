/**
 * [1481] Least Number of Unique Integers after K Removals
 *
 * Given an array of integers arr and an integer k. Find the least number of unique integers after removing exactly k elements.
 *
 * <ol>
 * </ol>
 *
 *  
 * <strong class="example">Example 1:
 *
 *
 * Input: arr = [5,5,4], k = 1
 * Output: 1
 * Explanation: Remove the single 4, only 5 is left.
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: arr = [4,3,1,1,3,3,2], k = 3
 * Output: 2
 * Explanation: Remove 4, 2 and either one of the two 1s or three 3s. 1 and 3 will be left.
 *
 *  
 * Constraints:
 *
 *
 *     1 <= arr.length <= 10^5
 *     1 <= arr[i] <= 10^9
 *     0 <= k <= arr.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
		let mut cnts = std::collections::HashMap::new();
		for n in arr {
			*cnts.entry(n).or_insert(0) += 1;
		}
		let mut cnts = cnts.into_values().collect::<Vec<_>>();
		cnts.sort_unstable();
		let tot = cnts.len() as i32;
		for (i, c) in (0..).zip(cnts) {
			if c <= k {
				k -= c;
			} else {
				return tot - i;
			}
		}
		0
	}
}

// submission codes end
