/**
 * [1670] Design Front Middle Back Queue
 *
 * Design a queue that supports push and pop operations in the front, middle, and back.
 * Implement the FrontMiddleBack class:
 *
 *     FrontMiddleBack() Initializes the queue.
 *     void pushFront(int val) Adds val to the front of the queue.
 *     void pushMiddle(int val) Adds val to the middle of the queue.
 *     void pushBack(int val) Adds val to the back of the queue.
 *     int popFront() Removes the front element of the queue and returns it. If the queue is empty, return -1.
 *     int popMiddle() Removes the middle element of the queue and returns it. If the queue is empty, return -1.
 *     int popBack() Removes the back element of the queue and returns it. If the queue is empty, return -1.
 *
 * Notice that when there are two middle position choices, the operation is performed on the frontmost middle position choice. For example:
 *
 *     Pushing 6 into the middle of [1, 2, 3, 4, 5] results in [1, 2, <u>6</u>, 3, 4, 5].
 *     Popping the middle from [1, 2, <u>3</u>, 4, 5, 6] returns 3 and results in [1, 2, 4, 5, 6].
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input:
 * ["FrontMiddleBackQueue", "pushFront", "pushBack", "pushMiddle", "pushMiddle", "popFront", "popMiddle", "popMiddle", "popBack", "popFront"]
 * [[], [1], [2], [3], [4], [], [], [], [], []]
 * Output:
 * [null, null, null, null, null, 1, 3, 4, 2, -1]
 * Explanation:
 * FrontMiddleBackQueue q = new FrontMiddleBackQueue();
 * q.pushFront(1);   // [<u>1</u>]
 * q.pushBack(2);    // [1, <u>2</u>]
 * q.pushMiddle(3);  // [1, <u>3</u>, 2]
 * q.pushMiddle(4);  // [1, <u>4</u>, 3, 2]
 * q.popFront();     // return 1 -> [4, 3, 2]
 * q.popMiddle();    // return 3 -> [4, 2]
 * q.popMiddle();    // return 4 -> [2]
 * q.popBack();      // return 2 -> []
 * q.popFront();     // return -1 -> [] (The queue is empty)
 *
 *  
 * Constraints:
 *
 *     1 <= val <= 10^9
 *     At most 1000 calls will be made to pushFront, pushMiddle, pushBack, popFront, popMiddle, and popBack.
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::VecDeque;
struct FrontMiddleBackQueue(VecDeque<i32>, VecDeque<i32>);

impl FrontMiddleBackQueue {
	fn new() -> Self { Self(VecDeque::new(), VecDeque::new()) }

	fn push_front(&mut self, val: i32) {
		self.0.push_front(val);
		self.balance();
	}

	fn push_middle(&mut self, val: i32) {
		self.0.push_back(val);
		self.balance();
	}

	fn push_back(&mut self, val: i32) {
		self.1.push_back(val);
		self.balance();
	}

	fn pop_front(&mut self) -> i32 {
		let ans = self
			.0
			.pop_front()
			.or_else(|| self.1.pop_front())
			.unwrap_or(-1);
		self.balance();
		ans
	}

	fn pop_middle(&mut self) -> i32 {
		if self.0.len() == self.1.len() {
			let ans = self.0.pop_back().unwrap_or(-1);
			self.balance();
			ans
		} else {
			self.1.pop_front().unwrap()
		}
	}

	fn pop_back(&mut self) -> i32 {
		let ans = self.1.pop_back().unwrap_or(-1);
		self.balance();
		ans
	}
	// XXX: assure self.1.len() <= self.0.len() <= self.1.len() + 1
	fn balance(&mut self) {
		while self.0.len() + 1 < self.1.len() {
			self.0.push_back(self.1.pop_front().unwrap());
		}
		while self.0.len() > self.1.len() {
			self.1.push_front(self.0.pop_back().unwrap());
		}
	}
}

// submission codes end
