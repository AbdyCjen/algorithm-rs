/**
 * [963] Three Equal Parts
 *
 * Given an array A of 0s and 1s, divide the array into 3 non-empty parts such that all of these parts represent the same binary value.
 *
 * If it is possible, return any [i, j] with i+1 < j, such that:
 *
 *
 * 	A[0], A[1], ..., A[i] is the first part;
 * 	A[i+1], A[i+2], ..., A[j-1] is the second part, and
 * 	A[j], A[j+1], ..., A[A.length - 1] is the third part.
 * 	All three parts have equal binary value.
 *
 *
 * If it is not possible, return [-1, -1].
 *
 * Note that the entire part is used when considering what binary value it represents.  For example, [1,1,0] represents 6 in decimal, not 3.  Also, leading zeros are allowed, so [0,1,1] and [1,1] represent the same value.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">[1,0,1,0,1]</span>
 * Output: <span id="example-output-1">[0,3]</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[1,1,0,1,1]</span>
 * Output: <span id="example-output-2">[-1,-1]</span>
 * </div>
 *
 *  
 *
 * Note:
 *
 * <ol>
 * 	3 <= A.length <= 30000
 * 	A[i] == 0 or A[i] == 1
 * </ol>
 *
 * <div>
 * <div> </div>
 * </div>
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	// divide ones into 3 part
	pub fn three_equal_parts(a: Vec<i32>) -> Vec<i32> {
		fn three_equal_parts_inner(a: Vec<i32>) -> Option<(i32, i32)> {
			let total_ones: i32 = a.iter().sum();
			if total_ones % 3 != 0 {
				return None;
			}
			if total_ones == 0 {
				return Some((0, a.len() as i32 - 1));
			}
			let part_ones = total_ones / 3;
			let mut it = a.iter().enumerate().filter(|(_, &c)| c == 1);
			let begin1 = it.next()?.0;
			let begin2 = it.nth(part_ones as usize - 1)?.0;
			let begin3 = it.nth(part_ones as usize - 1)?.0;
			let part_len = a.len() - begin3;
			if &a[begin1..begin1 + part_len] == &a[begin2..begin2 + part_len]
				&& &a[begin2..begin2 + part_len] == &a[begin3..begin3 + part_len]
			{
				Some(((begin1 + part_len - 1) as i32, (begin2 + part_len) as i32))
			} else {
				None
			}
		}
		three_equal_parts_inner(a)
			.map(|tp| vec![tp.0, tp.1])
			.unwrap_or(vec![-1, -1])
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_963() {
		let test_wrapper = |input, output| assert_eq!(Solution::three_equal_parts(input), output);
		test_wrapper(vec![1, 1, 1], vec![0, 2]);
		test_wrapper(vec![0, 1, 0], vec![-1, -1]);
		test_wrapper(vec![1, 1, 1, 1, 1, 1], vec![1, 4]);
		test_wrapper(vec![1, 0, 1, 0, 1], vec![0, 3]);
		test_wrapper(vec![1, 1, 0, 1, 1], vec![-1, -1]);
		test_wrapper(vec![1, 1, 0, 1, 0], vec![-1, -1]);
		test_wrapper(vec![1, 0, 1, 0, 1, 0], vec![1, 4]);
		test_wrapper(vec![1, 0, 1, 0, 0, 1, 0], vec![1, 4]);
		test_wrapper(vec![1, 1, 1, 0], vec![-1, -1]);
		test_wrapper(vec![0, 0, 0, 0, 0], vec![0, 4]);
	}
}
