/**
 * [67] Add Binary
 *
 * Given two binary strings a and b, return their sum as a binary string.
 *  
 * <strong class="example">Example 1:
 * Input: a = "11", b = "1"
 * Output: "100"
 * <strong class="example">Example 2:
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 *  
 * Constraints:
 *
 *     1 <= a.length, b.length <= 10^4
 *     a and b consist only of '0' or '1' characters.
 *     Each string does not contain leading zeros except for the zero itself.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn add_binary(a: String, b: String) -> String {
		let mut flow = 0;
		let mut ans = Vec::with_capacity(a.len().max(b.len()) + 1);
		let mut its = (a.bytes().rev(), b.bytes().rev());
		loop {
			match (its.0.next(), its.1.next()) {
				(Some(a), Some(b)) => {
					let s = (a - b'0') + (b - b'0') + flow;
					ans.push(s % 2);
					flow = (s >= 2) as u8;
				}
				(Some(a), _) | (_, Some(a)) => {
					let a = a - b'0';
					ans.push(a ^ flow);
					flow &= a;
				}
				_ => break,
			}
		}
		if flow > 0 {
			ans.push(flow);
		}
		ans.reverse();

		for c in &mut ans {
			*c += b'0';
		}
		String::from_utf8(ans).unwrap()
	}
}

// submission codes end
