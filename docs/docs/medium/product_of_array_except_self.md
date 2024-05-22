---
sidebar_position: 3
---

# Product of Array Except Self
Given an integer array `nums`, return an array `answer` such that `answer[i]` is equal to the product of all the elements of `nums` except `nums[i]`.

The product of any prefix or suffix of `nums` is **guaranteed** to fit in a **32-bit integer**.

You must write an algorithm that runs in `O(n)` time and without using the division operation.

## Examples
### Example 1
> `Input: nums = [1,2,3,4]` <br />
> `Output: [24,12,8,6]`

### Example 2
> `Input: nums = [-1,1,0,-3,3]` <br />
> `Output: [0,0,9,0,0]`

## Notes
### Conceptual Idea
- We can create prefix and suffix product arrays for the input to solve the problem
- Initialize 2 arrays, prefx and suffix.
- The prefix array houses the products of the array before the value, the suffix array houses the products of the array after the value
- For the prefix array, we add a `1` to the start of the array
  - Iterate over the numbers and for each number multiply it with the previous value in the array
- For the suffix array, we add a `1` to the end of the array
  - Iterate over the numbers backwards and for each number multiply it with the value ahead of it in the array
- Both the suffix and prefix array will be 1 longer than the input array
- To get the result, initialize two pointers, one at the start of the prefix array and the other at the second value of the suffix array
- Multiply the values at the pointers and add to the result array, iterate until the end of the suffix array

### Complexity
#### Time
- We only iterate over the values once -> O(n)

#### Space
- We create 2 arrays of length `n` for the suffix and prefix -> O(n)

## Solution
### Rust
```rust
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
  let n = nums.len();
  let mut prefix: Vec<i32> = vec![1; n + 1];
  let mut suffix: Vec<i32> = vec![1; n + 1];
  for (i, num) in nums.iter().enumerate() {
    prefix[i + 1] = prefix[i] * num;
  }
  for (i, num) in nums.iter().rev().enumerate() {
    suffix[n - i - 1] = suffix[n - i] * num;
  }
  let mut result: Vec<i32> = vec![];
  for i in 1..suffix.len() {
    result.push(prefix[i - 1] * suffix[i]);
  }
  return result;
}
```