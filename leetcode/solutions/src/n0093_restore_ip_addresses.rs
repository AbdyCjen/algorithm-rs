/**
 * [93] Restore IP Addresses
 *
 * A valid IP address consists of exactly four integers separated by single dots. Each integer is between 0 and 255 (inclusive) and cannot have leading zeros.
 *
 *     For example, "0.1.2.201" and "192.168.1.1" are valid IP addresses, but "0.011.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP addresses.
 *
 * Given a string s containing only digits, return all possible valid IP addresses that can be formed by inserting dots into s. You are not allowed to reorder or remove any digits in s. You may return the valid IP addresses in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "25525511135"
 * Output: ["255.255.11.135","255.255.111.35"]
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "0000"
 * Output: ["0.0.0.0"]
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "101023"
 * Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 20
 *     s consists of digits only.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn restore_ip_addresses(s: String) -> Vec<String> {
		let mut ans = vec![];
		Self::solve(&s, 4, &mut vec![], &mut ans);
		ans
	}

	fn solve<'a>(s: &'a str, cnt: i32, st: &mut Vec<&'a str>, ans: &mut Vec<String>) {
		if s.is_empty() || cnt == 0 {
			if s.is_empty() && cnt == 0 {
				ans.push(st.join("."));
			}
			return;
		}
		{
			let (digit, s) = s.split_at(1);
			st.push(digit);
			Self::solve(s, cnt - 1, st, ans);
			st.pop();
		}

		if let Some(digit) = s.get(..2) {
			if &s[..1] != "0" {
				st.push(digit);
				Self::solve(&s[2..], cnt - 1, st, ans);
				st.pop();
			}
		}

		if let Some(digit) = s.get(..3) {
			if &s[..1] != "0" && digit <= "255" {
				st.push(digit);
				Self::solve(&s[3..], cnt - 1, st, ans);
				st.pop();
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_93() {
		assert_eq!(
			Solution::restore_ip_addresses("25525511135".to_owned()),
			vec_string!["255.255.11.135", "255.255.111.35"]
		);
	}
}
