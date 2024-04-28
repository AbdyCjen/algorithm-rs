/**
 * [954] Maximum Sum Circular Subarray
 *
 * Given a circular array C of integers represented by A, find the maximum possible sum of a non-empty subarray of C.
 *
 * Here, a circular array means the end of the array connects to the beginning of the array.  (Formally, C[i] = A[i] when 0 <= i < A.length, and C[i+A.length] = C[i] when i >= 0.)
 *
 * Also, a subarray may only include each element of the fixed buffer A at most once.  (Formally, for a subarray C[i], C[i+1], ..., C[j], there does not exist i <= k1, k2 <= j with k1 % A.length = k2 % A.length.)
 *
 *  
 *
 * <div>
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">[1,-2,3,-2]</span>
 * Output: <span id="example-output-1">3
 * Explanation: Subarray [3] has maximum sum 3</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[5,-3,5]</span>
 * Output: <span id="example-output-2">10
 * </span><span id="example-output-3">Explanation: </span><span id="example-output-1">Subarray [5,5] has maximum sum </span><span>5 + 5 = 10</span>
 *
 *
 * <div>
 * Example 3:
 *
 *
 * Input: <span id="example-input-3-1">[3,-1,2,-1]</span>
 * Output: <span id="example-output-3">4
 * Explanation: </span><span id="example-output-1">Subarray [2,-1,3] has maximum sum </span><span>2 + (-1) + 3 = 4</span>
 *
 *
 * <div>
 * Example 4:
 *
 *
 * Input: <span id="example-input-4-1">[3,-2,2,-3]</span>
 * Output: <span id="example-output-4">3
 * </span><span id="example-output-3">Explanation: </span><span id="example-output-1">Subarray [3] and [3,-2,2] both have maximum sum </span><span>3</span>
 *
 *
 * Example 5:
 *
 *
 * Input: <span id="example-input-5-1">[-2,-3,-1]</span>
 * Output: <span id="example-output-5">-1
 * </span><span id="example-output-3">Explanation: </span><span id="example-output-1">Subarray [-1] has maximum sum -1</span>
 *
 *
 *  
 *
 * Note:
 *
 * <ol>
 * -30000 <= A[i] <= 30000
 * 1 <= A.length <= 30000
 * </ol>
 * </div>
 * </div>
 * </div>
 * </div>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
		let mut sum_max_pref = Vec::new();
		a.iter().fold((0, i32::MIN), |(mut sum, mut max_sum), &n| {
			sum += n;
			max_sum = std::cmp::max(sum, max_sum);
			sum_max_pref.push(max_sum);
			(sum, max_sum)
		});

		let mut sum_max_post = vec![0];
		a.iter()
			.rev()
			.fold((0, i32::MIN), |(mut sum, mut max_sum), &n| {
				sum += n;
				max_sum = std::cmp::max(sum, max_sum);
				sum_max_post.push(max_sum);
				(sum, max_sum)
			});
		sum_max_post.reverse();

		let mut sum_no_cir = i32::MIN;
		sum_max_pref
			.into_iter()
			.zip(sum_max_post.into_iter().skip(1))
			.zip(a)
			.map(|((a, b), n)| {
				sum_no_cir = match sum_no_cir {
					sum @ 1..=i32::MAX => sum + n,
					_ => n,
				};
				std::cmp::max(a + b, sum_no_cir)
			})
			.max()
			.unwrap_or(0)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_954() {
		assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
		assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
		assert_eq!(Solution::max_subarray_sum_circular(vec![3, -1, 2, -1]), 4);
		assert_eq!(Solution::max_subarray_sum_circular(vec![3, -2, 2, -3]), 3);
		assert_eq!(Solution::max_subarray_sum_circular(vec![-2, -3, -1]), -1);
	}
}
