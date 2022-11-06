/**
 * [433] Minimum Genetic Mutation
 *
 * A gene string can be represented by an 8-character long string, with choices from 'A', 'C', 'G', and 'T'.
 * Suppose we need to investigate a mutation from a gene string start to a gene string end where one mutation is defined as one single character changed in the gene string.
 *
 *     For example, "AACCGGTT" --> "AACCGGTA" is one mutation.
 *
 * There is also a gene bank bank that records all the valid gene mutations. A gene must be in bank to make it a valid gene string.
 * Given the two gene strings start and end and the gene bank bank, return the minimum number of mutations needed to mutate from start to end. If there is no such a mutation, return -1.
 * Note that the starting point is assumed to be valid, so it might not be included in the bank.
 *  
 * <strong class="example">Example 1:
 *
 * Input: start = "AACCGGTT", end = "AACCGGTA", bank = ["AACCGGTA"]
 * Output: 1
 *
 * <strong class="example">Example 2:
 *
 * Input: start = "AACCGGTT", end = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
 * Output: 2
 *
 * <strong class="example">Example 3:
 *
 * Input: start = "AAAAACCC", end = "AACCCCCC", bank = ["AAAACCCC","AAACCCCC","AACCCCCC"]
 * Output: 3
 *
 *  
 * Constraints:
 *
 *     start.length == 8
 *     end.length == 8
 *     0 <= bank.length <= 10
 *     bank[i].length == 8
 *     start, end, and bank[i] consist of only the characters ['A', 'C', 'G', 'T'].
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
		let mut dq = Vec::new();
		let mut to_visit = (0..bank.len()).collect::<Vec<_>>();
		let diff_count = |s1: &str, s2: &str| {
			s1.bytes()
				.zip(s2.bytes())
				.filter(|(c1, c2)| c1 != c2)
				.count()
		};
		dq.push(&start);

		let mut ans = 0;
		while !dq.is_empty() {
			for s in dq.split_off(0) {
				let mut i = 0;
				if s == &end {
					return ans;
				}

				while i < to_visit.len() {
					let ss = &bank[to_visit[i]];
					if diff_count(s, ss) == 1 {
						dq.push(ss);
						to_visit.swap_remove(i);
					} else {
						i += 1;
					}
				}
			}
			ans += 1;
		}

		-1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_433() {
		assert_eq!(
			Solution::min_mutation(
				"AACCGGTT".to_owned(),
				"AACCGGTA".to_owned(),
				vec_string!("AACCGGTA")
			),
			1
		);
		assert_eq!(
			Solution::min_mutation("AACCGGTT".to_owned(), "AACCGGTA".to_owned(), vec_string!()),
			-1
		);
		assert_eq!(
			Solution::min_mutation(
				"AACCGGTT".to_owned(),
				"AAACGGTA".to_owned(),
				vec_string!["AACCGGTA", "AACCGCTA", "AAACGGTA"]
			),
			2
		);
	}
}
