---
sidebar_position: 3
---

# Contains Duplicate
## Problem Description
Given an integer array `nums`, return `true` if any value appears **at least twice** in the array, and return `false` if every element is distinct.

## Examples
### Example 1
> `Input: nums = [1,2,3,1]` <br />
> `Output: true`

### Example 2
> `Input: nums = [1,2,3,4]` <br />
> `Output: false`

### Example 2
> `Input: nums = [1,1,1,3,3,4,3,2,4,2]` <br />
> `Output: true`

## Notes
### Conceptual Idea
- Create a hashmap
- Iterater over the values
  + If the value is in the hashmap, return `true` (duplicate found)
  + If not, add it to the hashmap
- If we get to the end of the values, there are no duplicates, return `false`

### Complexity
### Time
- At worst case, iterate over the entire input
  + Results in O(n) time

### Space
- At worse case, add all the values to the hashmap, creating `n` entries
  + Results in O(n) space


## Solution
### Rust
```rust
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
    for num in nums.iter() {
      if hash.contains_key(&num) {
        return true;
      } else {
        hash.insert(*num, true);
      }
    }

    return false;
  }
```