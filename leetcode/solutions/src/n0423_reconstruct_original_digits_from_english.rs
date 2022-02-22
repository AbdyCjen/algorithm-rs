/**
 * [423] Reconstruct Original Digits from English
 *
 * Given a string s containing an out-of-order English representation of digits 0-9, return the digits in ascending order.
 *  
 * Example 1:
 * Input: s = "owoztneoer"
 * Output: "012"
 * Example 2:
 * Input: s = "fviefuro"
 * Output: "45"
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s[i] is one of the characters ["e","g","f","i","h","o","n","s","r","u","t","w","v","x","z"].
 *     s is guaranteed to be valid.
 *
 */
pub struct Solution {}

// submission codes start here
/*
6 x six
8 g eight
4 u four
2 w two
7 s seven
5 v five
1 o one
3 r three
9 e nine
 */
#[allow(dead_code)]
impl Solution {
	pub fn original_digits(s: String) -> String {
		const IDX: [(u8, u8, &str); 10] = [
			(0, b'z', "zero"),
			(6, b'x', "six"),
			(8, b'g', "eight"),
			(4, b'u', "four"),
			(2, b'w', "two"),
			(7, b's', "seven"),
			(5, b'v', "five"),
			(1, b'o', "one"),
			(3, b'r', "three"),
			(9, b'e', "nine"),
		];

		let mut cnts = [0_i32; 26];
		for c in s.into_bytes() {
			cnts[(c - b'a') as usize] += 1;
		}

		let mut ans = [0; 10];
		for (i, c, s) in &IDX {
			let cnt = cnts[(c - b'a') as usize];
			for cc in s.bytes() {
				cnts[(cc - b'a') as usize] -= cnt;
			}

			ans[*i as usize] = cnt;
		}

		let mut ans_str = Vec::new();
		for (i, &c) in ans.iter().enumerate() {
			for _ in 0..c {
				ans_str.push(b'0' + i as u8);
			}
		}

		String::from_utf8(ans_str).unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_423() {
		assert_eq!(Solution::original_digits("owoztneoer".to_owned()), "012");
	}
}
