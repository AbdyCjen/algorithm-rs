/**
 * [1039] Find the Town Judge
 *
 * In a town, there are N people labelled from 1 to N.  There is a rumor that one of these people is secretly the town judge.
 *
 * If the town judge exists, then:
 *
 * <ol>
 * The town judge trusts nobody.
 * Everybody (except for the town judge) trusts the town judge.
 * There is exactly one person that satisfies properties 1 and 2.
 * </ol>
 *
 * You are given trust, an array of pairs trust[i] = [a, b] representing that the person labelled a trusts the person labelled b.
 *
 * If the town judge exists and can be identified, return the label of the town judge.  Otherwise, return -1.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: N = <span id="example-input-1-1">2</span>, trust = <span id="example-input-1-2">[[1,2]]</span>
 * Output: <span id="example-output-1">2</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: N = <span id="example-input-2-1">3</span>, trust = <span id="example-input-2-2">[[1,3],[2,3]]</span>
 * Output: <span id="example-output-2">3</span>
 *
 *
 * <div>
 * Example 3:
 *
 *
 * Input: N = <span id="example-input-3-1">3</span>, trust = <span id="example-input-3-2">[[1,3],[2,3],[3,1]]</span>
 * Output: <span id="example-output-3">-1</span>
 *
 *
 * <div>
 * Example 4:
 *
 *
 * Input: N = <span id="example-input-4-1">3</span>, trust = <span id="example-input-4-2">[[1,2],[2,3]]</span>
 * Output: <span id="example-output-4">-1</span>
 *
 *
 * <div>
 * Example 5:
 *
 *
 * Input: N = <span id="example-input-5-1">4</span>, trust = <span id="example-input-5-2">[[1,3],[1,4],[2,3],[2,4],[4,3]]</span>
 * Output: <span id="example-output-5">3</span>
 *
 *  
 * </div>
 * </div>
 * </div>
 * </div>
 *
 * Note:
 *
 * <ol>
 * 1 <= N <= 1000
 * trust.length <= 10000
 * trust[i] are all different
 * trust[i][0] != trust[i][1]
 * 1 <= trust[i][0], trust[i][1] <= N
 * </ol>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
		let mut cnts = vec![0; n as usize + 1];
		cnts[0] = -1;
		for tr in &trust {
			cnts[tr[1] as usize] += 1;
		}
		match cnts.iter().position(|v| *v == n - 1) {
			Some(n) if trust.iter().all(|v| v[0] != n as i32) => n as i32,
			_ => -1,
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_997() {
		assert_eq!(Solution::find_judge(1, vec![]), 1);
		assert_eq!(Solution::find_judge(2, matrix![[1, 2]]), 2);
		assert_eq!(Solution::find_judge(3, matrix![[1, 3], [2, 3]]), 3);
		assert_eq!(Solution::find_judge(3, matrix![[1, 3], [2, 3], [3, 1]]), -1);
		assert_eq!(Solution::find_judge(3, matrix![[1, 2], [2, 3]]), -1);
		assert_eq!(
			Solution::find_judge(4, matrix![[1, 3], [1, 4], [2, 3], [2, 4], [4, 3]]),
			3
		);
	}
}
