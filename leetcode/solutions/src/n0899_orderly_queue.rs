/**
 * [899] Orderly Queue
 *
 * You are given a string s and an integer k. You can choose one of the first k letters of s and append it at the end of the string..
 * Return the lexicographically smallest string you could have after applying the mentioned step any number of moves.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "cba", k = 1
 * Output: "acb"
 * Explanation:
 * In the first move, we move the 1^st character 'c' to the end, obtaining the string "bac".
 * In the second move, we move the 1^st character 'b' to the end, obtaining the final result "acb".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "baaca", k = 3
 * Output: "aaabc"
 * Explanation:
 * In the first move, we move the 1^st character 'b' to the end, obtaining the string "aacab".
 * In the second move, we move the 3^rd character 'c' to the end, obtaining the final result "aaabc".
 *
 *  
 * Constraints:
 *
 *     1 <= k <= s.length <= 1000
 *     s consist of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn orderly_queue(s: String, k: i32) -> String {
		let mut s = s.into_bytes();
		if k > 1 {
			s.sort_unstable();
			String::from_utf8(s).unwrap()
		} else {
			let mut it = s.iter().cycle();
			let mut ans = it.clone();
			for _ in 1..s.len() {
				it.next();
				if ans.clone().gt(it.clone().take(s.len())) {
					ans = it.clone();
				}
			}
			String::from_utf8(ans.copied().take(s.len()).collect()).unwrap()
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_899() {
		assert_eq!(
			Solution::orderly_queue("cba".to_owned(), 1),
			"acb".to_owned()
		);
		assert_eq!(
			Solution::orderly_queue("baaca".to_owned(), 3),
			"aaabc".to_owned()
		);
		assert_eq!(
			Solution::orderly_queue("baaca".to_owned(), 2),
			"aaabc".to_owned()
		);
	}
}
