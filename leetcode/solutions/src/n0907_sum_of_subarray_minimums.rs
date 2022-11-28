/**
 * [907] Sum of Subarray Minimums
 *
 * Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous) subarray of arr. Since the answer may be large, return the answer modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [3,1,2,4]
 * Output: 17
 * Explanation:
 * Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4].
 * Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
 * Sum is 17.
 *
 * <strong class="example">Example 2:
 *
 * Input: arr = [11,81,94,43,3]
 * Output: 444
 *
 *  
 * Constraints:
 *
 *     1 <= arr.length <= 3 * 10^4
 *     1 <= arr[i] <= 3 * 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
		let mut ans = 0;
		let mut st = vec![(i32::MIN, -1)];
		let mut prv = vec![];
		for (&n, i) in arr.iter().zip(0..) {
			while n < st.last().unwrap().0 {
				st.pop();
			}
			prv.push(i - st.last().unwrap().1);
			st.push((n, i));
		}
		let mut st = vec![(i32::MIN, arr.len())];
		let mut post = vec![];
		for (i, &n) in arr.iter().enumerate().rev() {
			while n <= st.last().unwrap().0 {
				st.pop();
			}
			post.push((st.last().unwrap().1 - i) as i64);
			st.push((n, i));
		}
		post.reverse();

		for ((n, prv), post) in arr.into_iter().zip(prv).zip(post) {
			ans = (prv * post * n as i64 + ans) % (1e9 as i64 + 7);
		}

		ans as i32
	}

	pub fn sum_subarray_mins_01(arr: Vec<i32>) -> i32 {
		let mut ans = 0;
		for (i, n) in arr.iter().enumerate() {
			let prv = arr[..i].iter().rev().take_while(|m| *m > n).count() as i64 + 1;
			let post = arr[i + 1..].iter().take_while(|m| *m >= n).count() as i64 + 1;
			ans = (prv * post * *n as i64 + ans) % (1e9 as i64 + 7);
		}
		ans as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_907() {
		assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
		assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
	}
}
