/**
 * [763] Partition Labels
 *
 * You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.
 * Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.
 * Return a list of integers representing the size of these parts.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "ababcbacadefegdehijhklij"
 * Output: [9,7,8]
 * Explanation:
 * The partition is "ababcbaca", "defegde", "hijhklij".
 * This is a partition so that each letter appears in at most one part.
 * A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "eccbbbbdec"
 * Output: [10]
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 500
 *     s consists of lowercase English letters.
 *
 */
pub struct Solution {}

impl Solution {
	pub fn partition_labels(s: String) -> Vec<i32> {
		let mut occ = [None; 26];
		for (c, i) in s.bytes().zip(0..) {
			occ[(c - b'a') as usize].get_or_insert((i, 0)).1 = i + 1;
		}
		occ.sort_unstable();
		let (mut ans, mut rng) = (vec![], (0, 1));
		for &(b, e) in occ.iter().flatten() {
			if b < rng.1 {
				rng.1 = rng.1.max(e);
			} else {
				ans.push(rng.1 - rng.0);
				rng = (b, e);
			}
		}
		ans.push(rng.1 - rng.0);
		ans
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_763() {
		assert_eq!(
			Solution::partition_labels("ababcbacadefegdehijhklij".to_owned()),
			vec![9, 7, 8]
		);
		assert_eq!(
			Solution::partition_labels("eccbbbbdec".to_owned()),
			vec![10]
		);
	}
}
