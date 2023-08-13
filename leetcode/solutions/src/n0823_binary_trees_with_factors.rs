/**
 * [823] Binary Trees With Factors
 *
 * Given an array of unique integers, arr, where each integer arr[i] is strictly greater than 1.
 * We make a binary tree using these integers, and each number may be used for any number of times. Each non-leaf node's value should be equal to the product of the values of its children.
 * Return the number of binary trees we can make. The answer may be too large so return the answer modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [2,4]
 * Output: 3
 * Explanation: We can make these trees: [2], [4], [4, 2, 2]
 * <strong class="example">Example 2:
 *
 * Input: arr = [2,4,5,10]
 * Output: 7
 * Explanation: We can make these trees: [2], [4], [5], [10], [4, 2, 2], [10, 2, 5], [10, 5, 2].
 *  
 * Constraints:
 *
 *     1 <= arr.length <= 1000
 *     2 <= arr[i] <= 10^9
 *     All the values of arr are unique.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
		let mut cnts = vec![1; arr.len()];
		arr.sort_unstable();
		const MO: i64 = 1e9 as i64 + 7;
		for (i, n) in (1..).zip(&arr[1..]) {
			for (j, o) in (0..).zip(&arr[..i]).filter(|v| n % v.1 == 0) {
				if let Ok(k) = arr[..i].binary_search(&(n / o)) {
					cnts[i] = (cnts[i] + cnts[j] * cnts[k]) % MO;
				}
			}
		}
		cnts.into_iter().fold(0, |a, n| (a + n) % MO) as i32
	}
}

// submission codes end
