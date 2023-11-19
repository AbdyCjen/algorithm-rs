/**
 * [715] Range Module
 *
 * A Range Module is a module that tracks ranges of numbers. Design a data structure to track the ranges represented as half-open intervals and query about them.
 * A half-open interval [left, right) denotes all the real numbers x where left <= x < right.
 * Implement the RangeModule class:
 *
 *     RangeModule() Initializes the object of the data structure.
 *     void addRange(int left, int right) Adds the half-open interval [left, right), tracking every real number in that interval. Adding an interval that partially overlaps with currently tracked numbers should add any numbers in the interval [left, right) that are not already tracked.
 *     boolean queryRange(int left, int right) Returns true if every real number in the interval [left, right) is currently being tracked, and false otherwise.
 *     void removeRange(int left, int right) Stops tracking every real number currently being tracked in the half-open interval [left, right).
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input
 * ["RangeModule", "addRange", "removeRange", "queryRange", "queryRange", "queryRange"]
 * [[], [10, 20], [14, 16], [10, 14], [13, 15], [16, 17]]
 * Output
 * [null, null, null, true, false, true]
 * Explanation
 * RangeModule rangeModule = new RangeModule();
 * rangeModule.addRange(10, 20);
 * rangeModule.removeRange(14, 16);
 * rangeModule.queryRange(10, 14); // return True,(Every number in [10, 14) is being tracked)
 * rangeModule.queryRange(13, 15); // return False,(Numbers like 14, 14.03, 14.17 in [13, 15) are not being tracked)
 * rangeModule.queryRange(16, 17); // return True, (The number 16 in [16, 17) is still being tracked, despite the remove operation)
 *
 *  
 * Constraints:
 *
 *     1 <= left < right <= 10^9
 *     At most 10^4 calls will be made to addRange, queryRange, and removeRange.
 *
 */
// submission codes start here
use std::collections::BTreeMap;
struct RangeModule(BTreeMap<i32, i32>);

impl RangeModule {
	fn new() -> Self { Self(Default::default()) }

	fn add_range(&mut self, mut l: i32, mut r: i32) {
		if let Some((&pl, &pr)) = self.0.range(..=l).next_back() {
			if pr >= l {
				l = pl;
				r = r.max(pr);
				self.0.remove(&l);
			}
		}
		while let Some((&al, &ar)) = self.0.range(l + 1..).next() {
			if al <= r {
				r = r.max(ar);
				self.0.remove(&al);
			} else {
				break;
			}
		}
		self.0.insert(l, r);
	}

	fn query_range(&mut self, l: i32, r: i32) -> bool {
		self.0
			.range(..=l)
			.next_back()
			.map(|(_, &pr)| pr >= r)
			.unwrap_or(false)
	}

	fn remove_range(&mut self, l: i32, r: i32) {
		if let Some((&pl, &pr)) = self.0.range(..l).next_back() {
			if pr >= l {
				self.0.insert(pl, l);
				if pr > r {
					self.0.insert(r, pr);
					return;
				}
			}
		}
		while let Some((&al, &ar)) = self.0.range(l..).next() {
			if ar <= r {
				self.0.remove(&al);
			} else {
				if al < r {
					self.0.remove(&al);
					self.0.insert(r, ar);
				}
				break;
			}
		}
	}
}

struct RangeModule1(Vec<(Option<bool>, bool, [usize; 2])>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule1 {
	fn new() -> Self { Self(vec![(Some(false), false, [0, 0])]) }

	fn add_range(&mut self, l: i32, r: i32) { self.set(l, r - 1, 1, 1e9 as i32, 0, true); }

	fn query_range(&mut self, l: i32, r: i32) -> bool { self.check(l, r - 1, 1, 1e9 as i32, 0) }

	fn remove_range(&mut self, l: i32, r: i32) { self.set(l, r - 1, 1, 1e9 as i32, 0, false); }

	fn insert(&mut self) -> usize {
		self.0.push((None, false, [0, 0]));
		self.0.len() - 1
	}

	fn pushdown(&mut self, p: usize) {
		//let flg = self.0[p].0.take().unwrap_or(self.0[p].1);
		if let Some(flg) = self.0[p].0.take() {
			if self.0[p].2[0] == 0 {
				self.0[p].2[0] = self.insert();
			}
			let ch = self.0[p].2[0];
			self.0[ch].0 = Some(flg);
			self.0[ch].1 = flg;

			if self.0[p].2[1] == 0 {
				self.0[p].2[1] = self.insert();
			}

			let ch = self.0[p].2[1];
			self.0[ch].0 = Some(flg);
			self.0[ch].1 = flg;
		}
	}

	fn set(&mut self, l: i32, r: i32, s: i32, t: i32, p: usize, set: bool) {
		if l <= s && t <= r {
			self.0[p].0 = Some(set);
			self.0[p].1 = set;
			return;
		}
		let m = (s + t) / 2;
		self.pushdown(p);
		if l <= m {
			self.set(l, r, s, m, self.0[p].2[0], set);
		}
		if r > m {
			self.set(l, r, m + 1, t, self.0[p].2[1], set);
		}
		let [lc, rc] = self.0[p].2;
		self.0[p].1 = self.0[lc].1 & self.0[rc].1;
	}

	fn check(&mut self, l: i32, r: i32, s: i32, t: i32, p: usize) -> bool {
		if l <= s && t <= r {
			return self.0[p].1;
		}
		let m = (s + t) / 2;
		self.pushdown(p);
		let mut flg = true;
		if l <= m {
			flg &= self.check(l, r, s, m, self.0[p].2[0]);
		}
		if r > m {
			flg &= self.check(l, r, m + 1, t, self.0[p].2[1]);
		}
		flg
	}
}

/**
 * Your RangeModule object will be instantiated and called as such:
 * let obj = RangeModule::new();
 * obj.add_range(left, right);
 * let ret_2: bool = obj.query_range(left, right);
 * obj.remove_range(left, right);
 */

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_715() {
		let mut tr = RangeModule::new();
		tr.add_range(10, 20);
		tr.remove_range(14, 16);
		assert!(tr.query_range(10, 14));
		assert!(!tr.query_range(13, 15));
		assert!(tr.query_range(16, 17));

		let mut tr = RangeModule::new();
		tr.add_range(6, 8);
		tr.remove_range(7, 8);
		tr.remove_range(8, 9);
		tr.add_range(8, 9);
		tr.remove_range(1, 3);
		tr.add_range(1, 8);
		assert!(tr.query_range(2, 4));
		assert!(tr.query_range(2, 9));
		assert!(tr.query_range(4, 6));
	}
}
