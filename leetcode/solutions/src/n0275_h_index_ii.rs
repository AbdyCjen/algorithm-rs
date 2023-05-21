/**
 * [275] H-Index II
 *
 * Given an array of integers citations where citations[i] is the number of citations a researcher received for their i^th paper and citations is sorted in ascending order, return the researcher's h-index.
 * According to the <a href="https://en.wikipedia.org/wiki/H-index" target="_blank">definition of h-index on Wikipedia</a>: The h-index is defined as the maximum value of h such that the given researcher has published at least h papers that have each been cited at least h times.
 * You must write an algorithm that runs in logarithmic time.
 *  
 * <strong class="example">Example 1:
 *
 * Input: citations = [0,1,3,5,6]
 * Output: 3
 * Explanation: [0,1,3,5,6] means the researcher has 5 papers in total and each of them had received 0, 1, 3, 5, 6 citations respectively.
 * Since the researcher has 3 papers with at least 3 citations each and the remaining two with no more than 3 citations each, their h-index is 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: citations = [1,2,100]
 * Output: 2
 *
 *  
 * Constraints:
 *
 *     n == citations.length
 *     1 <= n <= 10^5
 *     0 <= citations[i] <= 1000
 *     citations is sorted in ascending order.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn h_index(cits: Vec<i32>) -> i32 {
		let (len, begin) = (cits.len() as i32, &cits[0]);
		len - cits
			.partition_point(|v| *v < unsafe { len - (v as *const i32).offset_from(begin) as i32 })
			as i32
	}

	pub fn h_index1(citations: Vec<i32>) -> i32 {
		citations
			.iter()
			.zip((1..=citations.len() as i32).rev())
			.find(|(cit, cnt)| *cit >= cnt)
			.map(|(_, c)| c)
			.unwrap_or(0)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_275() {}
}
