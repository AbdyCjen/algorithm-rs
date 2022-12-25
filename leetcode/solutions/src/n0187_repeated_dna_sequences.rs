/**
 * [187] Repeated DNA Sequences
 *
 * The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.
 *
 *     For example, "ACGAATTCCG" is a DNA sequence.
 *
 * When studying DNA, it is useful to identify repeated sequences within the DNA.
 * Given a string s that represents a DNA sequence, return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule. You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * Input: s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
 * Output: ["AAAAACCCCC","CCCCCAAAAA"]
 * <strong class="example">Example 2:
 * Input: s = "AAAAAAAAAAAAA"
 * Output: ["AAAAAAAAAA"]
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s[i] is either 'A', 'C', 'G', or 'T'.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
		let mut visited = std::collections::HashMap::new();
		for w in s.as_bytes().windows(10) {
			*visited.entry(w).or_insert(0) += 1;
		}
		visited
			.into_iter()
			.filter(|(_, cnt)| *cnt > 1)
			.map(|s| std::str::from_utf8(s.0).unwrap().to_owned())
			.collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_187() {
		assert_eq!(
			Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_owned()),
			vec_string!["CCCCCAAAAA", "AAAAACCCCC"]
		);
		assert_eq!(
			Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_owned()),
			vec_string!["AAAAAAAAAA"]
		);
	}
}
