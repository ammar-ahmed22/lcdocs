---
sidebar_position: 1
---

# Two Sum
Given an array of integers `nums` and an integer `target`, return _indices of the two numbers such that they add up to `target`_.

You may assume that each input would have **exactly one solution**, and you may not use the same element twice.

You can return the answer in any order.

## Examples
### Example 1
> `Input: nums = [2,7,11,15], target = 9` <br />
> `Output: [0,1]`

### Example 2
> `Input: nums = [3,2,4], target = 6` <br />
> `Output: [1,2]`

### Example 3
> `Input: nums = [3,3], target = 6` <br />
> `Output: [0,1]`

## Notes
### Conceptual Idea
- Create a hashmap with the number as the key and the index as the value
- Iterate over the numbers
  + Calculate the number we are looking for at each iteration to make the target (`t = target - curr`)
  + Check if the hashmap contains that value
    - If it does, check if the index it's at is not the same as the current index (otherwise we'd be using the same element twice)
      + Return the current index and the index of the found value
    - If it doesn't, add the current value to the hashmap

### Complexity
#### Time
- We iterate over the numbers once -> O(n)

#### Space
- We only create the hashmap for extra space
  + At the worst case, we add all values to the hashmap -> O(n)

## Solution
### Rust
```rust
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut hash: HashMap<i32, usize> = HashMap::new();
  for (i, n) in nums.iter().enumerate() {
    let t = target - n;
    match hash.get(&t) {
      Some(val) => {
        if *val != i {
          return vec![*val as i32, i as i32]
        }
      }, 
      None => {
        hash.insert(*n, i);
      }
    }
  }
  return vec![0, 0]
}
```