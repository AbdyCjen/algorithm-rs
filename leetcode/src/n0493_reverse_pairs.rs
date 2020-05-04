/**
 * [493] Reverse Pairs
 *
 * Given an array nums, we call (i, j) an important reverse pair if i < j and nums[i] > 2*nums[j].
 *
 * You need to return the number of important reverse pairs in the given array.
 *
 * Example1:
 *
 * Input: [1,3,2,3,1]
 * Output: 2
 *
 *
 * Example2:
 *
 * Input: [2,4,3,5,1]
 * Output: 3
 *
 *
 * Note:<br>
 * <ol>
 * The length of the given array will not exceed 50,000.
 * All the numbers in the input array are in the range of 32-bit integer.
 * </ol>
 *
 */
pub struct Solution {}

// submission codes start here

// 杀妈了, 写完发现不是单纯的逆序对
#[allow(dead_code)]
impl Solution {
	pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
		// vec[(val, lcnt, rcnt)]
		let mut st = Vec::new();
		if let Some(&i) = nums.get(0) {
			st.push((i, 0, 0));
		} else {
			return 0;
		}

		let mut res = 0;
		for &i in nums.iter().skip(1) {
			let mut last_cnt = 0;
			while let Some(t) = st.last_mut() {
				if t.0 < i {
					st.push((i, last_cnt, 0));
					res += last_cnt;
					break;
				} else {
					let t = st.pop().unwrap();
					if let Some(tt) = st.last_mut() {
						tt.2 = t.1 + t.2 + 1;
					}
					last_cnt = t.1 + t.2 + 1;
				}
			}
			if st.is_empty() {
				res += last_cnt;
				st.push((i, last_cnt, 0));
			}
		}
		res += st.iter().rev().skip(1).fold(0, |c, &(_, lcnt, _)| c + lcnt);
		res
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_493() {
		assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 5);
		assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 5);
	}
}
