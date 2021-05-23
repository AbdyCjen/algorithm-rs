/**
 * [1553] Count Triplets That Can Form Two Arrays of Equal XOR
 *
 * Given an array of integers arr.
 *
 * We want to select three indices i, j and k where (0 <= i < j <= k < arr.length).
 *
 * Let's define a and b as follows:
 *
 *
 *     a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]
 *     b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]
 *
 *
 * Note that ^ denotes the bitwise-xor operation.
 *
 * Return the number of triplets (i, j and k) Where a == b.
 *
 *  
 * Example 1:
 *
 *
 * Input: arr = [2,3,1,6,7]
 * Output: 4
 * Explanation: The triplets are (0,1,2), (0,2,2), (2,3,4) and (2,4,4)
 *
 *
 * Example 2:
 *
 *
 * Input: arr = [1,1,1,1,1]
 * Output: 10
 *
 *
 * Example 3:
 *
 *
 * Input: arr = [2,3]
 * Output: 0
 *
 *
 * Example 4:
 *
 *
 * Input: arr = [1,3,5,7,9]
 * Output: 3
 *
 *
 * Example 5:
 *
 *
 * Input: arr = [7,11,12,9,5,2,7,17,22]
 * Output: 8
 *
 *
 *  
 * Constraints:
 *
 *
 *     1 <= arr.length <= 300
 *     1 <= arr[i] <= 10^8
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn count_triplets(arr: Vec<i32>) -> i32 {
		use std::collections::HashMap;

		let mut hm: HashMap<_, Vec<_>> = HashMap::new();
		hm.entry(0).or_default().push(0);
		let mut acc = 0;
		let mut total = 0;
		for (i, v) in arr.into_iter().enumerate() {
			acc ^= v;
			if let Some(idxs) = hm.get(&acc) {
				for j in idxs {
					total += i - j;
				}
			}

			hm.entry(acc).or_default().push(i + 1);
		}
		total as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1553() {
		assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 4);
		assert_eq!(Solution::count_triplets(vec![1, 1, 1, 1, 1]), 10);
		assert_eq!(Solution::count_triplets(vec![1, 1, 1, 1, 1]), 10);
		assert_eq!(Solution::count_triplets(vec![2, 3]), 0);
		assert_eq!(Solution::count_triplets(vec![1, 3, 5, 7, 9]), 3);
		assert_eq!(
			Solution::count_triplets(vec![7, 11, 12, 9, 5, 2, 7, 17, 22]),
			8
		);
	}
}
