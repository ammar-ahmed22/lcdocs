---
sidebar_position: 6
---

# Binary Search
Given an array of integers `nums` which is sorted in ascending order, and an integer `target`, write a function to search `target` in `nums`. If `target` exists, then return its index. Otherwise, return `-1`.

You must write an algorithm with `O(log n)` runtime complexity.

## Examples
### Example 1
> Input: `nums = [-1,0,3,5,9,12], target = 9`  
> Output: `4`

### Example 2
> Input: `nums = [-1,0,3,5,9,12], target = 2`  
> Output: `-1` 

## Notes
### Conceptual Idea
- Since the array is sorted, we can use this to our advantage
- Look at the middle number, if it is equal to the target -> return the index
- If it is smaller than the target, we need to only look at the left half (do the same thing again with left half)
- If it is greater than the target, we need to only look at the right half (do the same thing again with the left half)
- If we divide too small, we return -1 (does not exist)
- Initialize two pointers, left and right
  + Left is at the start of the array and right is at the end of the array
  + Iterate while left is smaller than or equal to right
  + Calculate the midpoint of left and right
  + If the midpoint is equal to the target -> return the index
  + If the midpoint is less than the target
    - Make the left pointer, 1 + midpoint
  + If the midpoint is greater than the target
    - Make the right pointer, 1 - midpoint


### Complexity
- Time: O(logn)
- Space: O(1)

## Solution
### Rust
```rust
fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0 as usize;
    let mut right = nums.len() - 1;
    while left < nums.len() && right < nums.len() && left <= right {
      let mid = (left + right) / 2;
      if nums[mid] == target {
        return mid as i32;
      } else if nums[mid] < target {
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    return -1;
}
```

**NOTE**: The additional checks in the while loop are due to the the Rust `usize` type. When trying to subtract from 0 with a `usize` type it wraps around to the largest value. E.g. 0 - 1 becomes a very large positive number instead of a negative number.