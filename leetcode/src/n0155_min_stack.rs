/**
 * [155] Min Stack
 *
 * Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
 *
 *
 * push(x) -- Push element x onto stack.
 * pop() -- Removes the element on top of the stack.
 * top() -- Get the top element.
 * getMin() -- Retrieve the minimum element in the stack.
 *
 *
 *  
 *
 * Example:
 *
 *
 * MinStack minStack = new MinStack();
 * minStack.push(-2);
 * minStack.push(0);
 * minStack.push(-3);
 * minStack.getMin();   --> Returns -3.
 * minStack.pop();
 * minStack.top();      --> Returns 0.
 * minStack.getMin();   --> Returns -2.
 *
 *
 *  
 *
 */
// submission codes start here
use std::default::Default;
#[derive(Default)]
struct MinStack {
	stk: Vec<i32>,
	mono_stk: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MinStack {
	/** initialize your data structure here. */
	fn new() -> Self { Default::default() }

	fn push(&mut self, x: i32) {
		self.stk.push(x);
		if x <= self.mono_stk.last().copied().unwrap_or(std::i32::MAX) {
			self.mono_stk.push(x)
		}
	}

	fn pop(&mut self) {
		if self.stk.pop() == self.mono_stk.last().copied() {
			self.mono_stk.pop();
		}
	}

	fn top(&self) -> i32 { *self.stk.last().unwrap() }

	fn get_min(&self) -> i32 { *self.mono_stk.last().unwrap() }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_155() {
		let mut min_stack = MinStack::new();
		min_stack.push(-2);
		min_stack.push(0);
		min_stack.push(-3);
		assert_eq!(min_stack.get_min(), -3);
		min_stack.pop();
		assert_eq!(min_stack.top(), 0);
		assert_eq!(min_stack.get_min(), -2);
	}
}
