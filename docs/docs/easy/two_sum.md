---
sidebar_position: 2
---

# Two Sum
## Problem Description
Given an array of integers `nums` and an integer `target`, return _indices of the two numbers such that they add up to `target`_.

You may assume that each input would have **exactly one solution**, and you may not use the same element twice.

You can return the answer in any order.

## Examples
### Example 1
> `Input: nums = [2,7,11,15], target = 9` <br />
> `Output: [0, 1]` <br />
> `Explanation: Because nums[0] + nums[1] = 9, we return [0, 1]`

### Example 2
> `Input: nums = [3,2,4], target = 6` <br />
> `Output: [1,2]`

### Example 3
> `Input: nums = [3,3], target = 6` <br />
> `Output: [0,1]`

## Notes
### Conceptual Idea
- Use a hashmap and store all the numbers with their indices
  + Doesn't matter if there are duplicates, overwrite with the last occurrence
- Iterate over the numbers
  + Calculate the value that we are looking for at each iteration `t = curr - target`
  + Check if the hashmap contains the value, we are looking for + check if the index for that value is not the current index (cannot use the same element twice)
    - If true, return array of the current index and the index in the hashmap

:::note

####  Why do duplicates when constructing the hashmap not matter?
As we can see in the second part of the solution, we are calculating the value we are looking for using the current value. Therefore, there is only a single value that can add up to the target.

e.g. `nums = [2, 4, 4], target = 6`,

When we iterate over the numbers and get to the first `2`, we are looking for a `4` in the array to make the target. It doesn't matter that there are other `4`'s in the array as long as we find one.

:::

### Complexity
#### Time
- We have two iterations over the array
  + First, to construct the hashmap -> O(n)
  + Second, to find the solution -> O(n)
- Results in O(n) time

#### Space
- The only additional space we are creating for this problem is the hashmap
  + In the worst case (all distinct values), we are inserting `n` entries.
- Results in O(n) space 

## Solution
### Rust
```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> UnorderedVec<i32> {
    let mut hash: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for (i, n) in nums.iter().enumerate() {
      hash.insert(*n, i as i32);
    }

    for (i, n) in nums.iter().enumerate() {
      let t = target - n;
      if hash.contains_key(&t) {
        let idx = match hash.get(&t) {
          Some(i) => *i,
          None => -1
        };
        if idx != -1 && idx != (i as i32) {
          return UnorderedVec(vec![idx, i as i32]);
        }
      }
    }

    return UnorderedVec(vec![0, 0])
  }
```

:::note

The solution returns `UnorderedVec` due to my custom testing implementation which holds equality even if the order of the values is different.

:::