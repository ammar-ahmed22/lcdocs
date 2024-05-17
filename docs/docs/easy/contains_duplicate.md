---
sidebar_position: 2
---

# Contains Duplicate
Given an integer array `nums`, return `true` if any value appears **at least twice** in the array, and return `false` if every element is distinct.

## Examples
### Example 1
> `Input: nums = [1,2,3,1]` <br />
> `Output: true`

### Example 2
> `Input: nums = [1,2,3,4]` <br />
> `Output: false`

### Example 3
> `Input: nums = [1,1,1,3,3,4,3,2,4,2]` <br />
> `Output: true`

## Notes
### Conceptual Idea
- Create a hashmap with the integer as the key and bool as the value
- Iterate over the numbers
  + Check if the current value is in the hashmap
    - If it is, return true
    - If it's not, add the value to hashmap
- Get to the end, return false

### Complexity
#### Time
- Iterate over values once, O(n)

#### Space
- Create a hashmap for every entry at the worst case, O(n)

## Solution
### Rust
```rust
fn contains_duplicate(nums: Vec<i32>) -> bool {
  let mut hash: HashMap<i32, bool> = HashMap::new();
  for num in nums.iter() {
    match hash.get(num) {
      Some(_) => {
        return true;
      },
      None => {
        hash.insert(*num, true);
      }
    }
  }
  return false;        
}
```