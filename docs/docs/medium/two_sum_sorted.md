---
sidebar_position: 6
---

# Two Sum II - Input Array Is Sorted
Given a **1-indexed** array of integers `numbers` that is already **sorted in non-decreasing order**, find two numbers such that they add up to a specific `target` number. Let these two numbers be `numbers[index1]` and `numbers[index2]` where `1 <= index1 < index2 <= numbers.length`.

Return the indices of the two numbers, `index1` and `index2`, **added by one** as an integer array `[index1, index2]` of length 2.

The tests are generated such that there is **exactly one solution**. You **may not** use the same element twice.

Your solution must use only constant extra space.

## Examples
### Example 1
> `Input: numbers = [2,7,11,15], target = 9` <br />
> `Output: [1, 2]`

### Example 2
> `Input: numbers = [2,3,4], target = 6` <br />
> `Output: [1, 3]`

### Example 3
> `Input: numbers = [-1,0], target = -1` <br />
> `Output: [1, 2]`

## Notes
### Conceptual Idea
- Create 2 pointers, one at the start of the array and one at the end
- Iterate while true (guaranteed solution)
  + On each iteration, add the values at the pointers
    - If it equals target, return the result
    - If it is less than the target, decrement the right pointer (smaller number)
    - If it is greater than the target, increment the left pointer (bigger number)


### Complexity
#### Time
- O(n)

#### Space
- O(1)

## Solution
### Rust
```rust
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0 as usize;
    let mut r = nums.len() - 1;

    while l < r {
      let curr = nums[l] + nums[r];
      if curr < target {
        l += 1;
      } else if curr > target {
        r -= 1;
      } else {
        return vec![(l + 1) as i32, (r + 1) as i32]
      }
    }
    return vec![0, 0];
}
```