/**
 * [2336] Smallest Number in Infinite Set
 *
 * You have a set which contains all positive integers [1, 2, 3, 4, 5, ...].
 * Implement the SmallestInfiniteSet class:
 *
 *     SmallestInfiniteSet() Initializes the SmallestInfiniteSet object to contain all positive integers.
 *     int popSmallest() Removes and returns the smallest integer contained in the infinite set.
 *     void addBack(int num) Adds a positive integer num back into the infinite set, if it is not already in the infinite set.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input
 * ["SmallestInfiniteSet", "addBack", "popSmallest", "popSmallest", "popSmallest", "addBack", "popSmallest", "popSmallest", "popSmallest"]
 * [[], [2], [], [], [], [1], [], [], []]
 * Output
 * [null, null, 1, 2, 3, null, 1, 4, 5]
 * Explanation
 * SmallestInfiniteSet smallestInfiniteSet = new SmallestInfiniteSet();
 * smallestInfiniteSet.addBack(2);    // 2 is already in the set, so no change is made.
 * smallestInfiniteSet.popSmallest(); // return 1, since 1 is the smallest number, and remove it from the set.
 * smallestInfiniteSet.popSmallest(); // return 2, and remove it from the set.
 * smallestInfiniteSet.popSmallest(); // return 3, and remove it from the set.
 * smallestInfiniteSet.addBack(1);    // 1 is added back to the set.
 * smallestInfiniteSet.popSmallest(); // return 1, since 1 was added back to the set and
 *                                    // is the smallest number, and remove it from the set.
 * smallestInfiniteSet.popSmallest(); // return 4, and remove it from the set.
 * smallestInfiniteSet.popSmallest(); // return 5, and remove it from the set.
 *
 *  
 * Constraints:
 *
 *     1 <= num <= 1000
 *     At most 1000 calls will be made in total to popSmallest and addBack.
 *
 */
pub struct Solution {}

// submission codes start here

//able to handle num in 0..
use std::collections::BinaryHeap;
struct SmallestInfiniteSet(BinaryHeap<i32>, i32);

impl SmallestInfiniteSet {
	fn new() -> Self { Self(BinaryHeap::new(), 0) }

	fn pop_smallest(&mut self) -> i32 {
		if let Some(ans) = self.0.pop() {
			while self.0.peek() == Some(&ans) {
				self.0.pop();
			}
			-ans
		} else {
			self.1 += 1;
			self.1
		}
	}

	fn add_back(&mut self, num: i32) {
		if num <= self.1 {
			self.0.push(-num)
		}
	}
}

// able to handle num < 1000
struct SmallestInfiniteSet1(BinaryHeap<i32>);

impl SmallestInfiniteSet1 {
	fn new() -> Self { Self(BinaryHeap::from((-1000..=-1).collect::<Vec<_>>())) }

	fn pop_smallest(&mut self) -> i32 {
		let ans = self.0.pop().unwrap();
		while self.0.peek() == Some(&ans) {
			self.0.pop();
		}
		-ans
	}

	fn add_back(&mut self, num: i32) { self.0.push(-num) }
}

// submission codes end
