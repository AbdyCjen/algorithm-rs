/**
 * [352] Data Stream as Disjoint Intervals
 *
 * Given a data stream input of non-negative integers a1, a2, ..., an, summarize the numbers seen so far as a list of disjoint intervals.
 * Implement the SummaryRanges class:
 *
 *     SummaryRanges() Initializes the object with an empty stream.
 *     void addNum(int value) Adds the integer value to the stream.
 *     int[][] getIntervals() Returns a summary of the integers in the stream currently as a list of disjoint intervals [starti, endi]. The answer should be sorted by starti.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input
 * ["SummaryRanges", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals"]
 * [[], [1], [], [3], [], [7], [], [2], [], [6], []]
 * Output
 * [null, null, [[1, 1]], null, [[1, 1], [3, 3]], null, [[1, 1], [3, 3], [7, 7]], null, [[1, 3], [7, 7]], null, [[1, 3], [6, 7]]]
 * Explanation
 * SummaryRanges summaryRanges = new SummaryRanges();
 * summaryRanges.addNum(1);      // arr = [1]
 * summaryRanges.getIntervals(); // return [[1, 1]]
 * summaryRanges.addNum(3);      // arr = [1, 3]
 * summaryRanges.getIntervals(); // return [[1, 1], [3, 3]]
 * summaryRanges.addNum(7);      // arr = [1, 3, 7]
 * summaryRanges.getIntervals(); // return [[1, 1], [3, 3], [7, 7]]
 * summaryRanges.addNum(2);      // arr = [1, 2, 3, 7]
 * summaryRanges.getIntervals(); // return [[1, 3], [7, 7]]
 * summaryRanges.addNum(6);      // arr = [1, 2, 3, 6, 7]
 * summaryRanges.getIntervals(); // return [[1, 3], [6, 7]]
 *
 *  
 * Constraints:
 *
 *     0 <= value <= 10^4
 *     At most 3 * 10^4 calls will be made to addNum and getIntervals.
 *
 *  
 * Follow up: What if there are lots of merges and the number of disjoint intervals is small compared to the size of the data stream?
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::BTreeSet;
#[derive(Default)]
struct SummaryRanges {
	intvs: BTreeSet<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
	fn new() -> Self { Self::default() }

	fn add_num(&mut self, value: i32) {
		let mut cur = (value, value);
		if let Some(&prv) = self.intvs.range(..(value, i32::MAX)).next_back() {
			if (prv.0..=prv.1).contains(&value) {
				return;
			} else if prv.1 + 1 == value {
				self.intvs.remove(&prv);
				cur = (prv.0, value);
			}
		}

		if let Some(&post) = self.intvs.range((value, 0)..).next() {
			if post.0 == value + 1 {
				self.intvs.remove(&post);
				cur.1 = post.1;
			}
		}
		self.intvs.insert(cur);
	}

	fn get_intervals(&self) -> Vec<Vec<i32>> {
		self.intvs.iter().map(|&(s, e)| vec![s, e]).collect()
	}
}

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(value);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_352() {}
}
