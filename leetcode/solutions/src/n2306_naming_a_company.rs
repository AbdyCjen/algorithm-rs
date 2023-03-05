/**
 * [2306] Naming a Company
 *
 * You are given an array of strings ideas that represents a list of names to be used in the process of naming a company. The process of naming a company is as follows:
 * <ol>
 *     Choose 2 distinct names from ideas, call them ideaA and ideaB.
 *     Swap the first letters of ideaA and ideaB with each other.
 *     If both of the new names are not found in the original ideas, then the name ideaA ideaB (the concatenation of ideaA and ideaB, separated by a space) is a valid company name.
 *     Otherwise, it is not a valid name.
 * </ol>
 * Return the number of distinct valid names for the company.
 *  
 * <strong class="example">Example 1:
 *
 * Input: ideas = ["coffee","donuts","time","toffee"]
 * Output: 6
 * Explanation: The following selections are valid:
 * - ("coffee", "donuts"): The company name created is "doffee conuts".
 * - ("donuts", "coffee"): The company name created is "conuts doffee".
 * - ("donuts", "time"): The company name created is "tonuts dime".
 * - ("donuts", "toffee"): The company name created is "tonuts doffee".
 * - ("time", "donuts"): The company name created is "dime tonuts".
 * - ("toffee", "donuts"): The company name created is "doffee tonuts".
 * Therefore, there are a total of 6 distinct company names.
 * The following are some examples of invalid selections:
 * - ("coffee", "time"): The name "toffee" formed after swapping already exists in the original array.
 * - ("time", "toffee"): Both names are still the same after swapping and exist in the original array.
 * - ("coffee", "toffee"): Both names formed after swapping already exist in the original array.
 *
 * <strong class="example">Example 2:
 *
 * Input: ideas = ["lack","back"]
 * Output: 0
 * Explanation: There are no valid selections. Therefore, 0 is returned.
 *
 *  
 * Constraints:
 *
 *     2 <= ideas.length <= 5 * 10^4
 *     1 <= ideas[i].length <= 10
 *     ideas[i] consists of lowercase English letters.
 *     All the strings in ideas are unique.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn distinct_names(ideas: Vec<String>) -> i64 {
		let mut cnts = vec![std::collections::HashSet::new(); 26];
		for w in &ideas {
			if let [c, s @ ..] = w.as_bytes() {
				cnts[(c - b'a') as usize].insert(s);
			}
		}
		let mut ans = 0;
		for i in 0..26 {
			for j in i + 1..26 {
				let inter = cnts[i].intersection(&cnts[j]).count() as i64;
				ans += (cnts[i].len() as i64 - inter) * (cnts[j].len() as i64 - inter);
			}
		}
		ans * 2
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2306() {
		assert_eq!(
			Solution::distinct_names(vec_string!["coffee", "donuts", "time", "toffee"]),
			6
		);
		assert_eq!(
			Solution::distinct_names(vec_string!["ac", "bc", "cc", "bb", "cb", "db"]),
			2
		);
	}
}
