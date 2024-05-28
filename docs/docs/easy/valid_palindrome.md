---
sidebar_position: 5
---

# Valid Palindrome
A phrase is a **palindrome** if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string `s`, return `true` if it is a **palindrome**, or `false` otherwise.

## Examples
### Example 1
> `Input: s = "A man, a plan, a canal: Panama"` <br />
> `Output: true`

### Example 2
> `Input: s = "race a car"` <br />
> `Output: false`

### Example 3
> `Input: s = " "` <br />
> `Output: true`

## Notes
### Conceptual Idea
- Initialize two pointers, one at the start and one at the end
- Iterate while the left pointer is less than or equal to the right (no crossover)
  + On each iteration, check if the value at the left pointer is alpha numeric, if it it's not keep moving it until it is. Same thing with the right (move backwards)
  + Compare the left and right values, if they are not the same, return false
- Get to the end, return true


### Complexity
#### Time
- Only iterate over the input once -> O(n)

#### Space
- No new space created -> O(1)

## Solution
### Rust
```rust
fn is_palindrome(s: String) -> bool {
  let mut l = 0 as usize;
  let mut r = s.len() - 1;
  let lowercase = s.to_lowercase();
  while l <= r {
    let left = lowercase.chars().nth(l).unwrap();
    let right = lowercase.chars().nth(r).unwrap();
    if !left.is_alphanumeric() {
      l += 1;
      continue;
    }

    if !right.is_alphanumeric() {
      r -= 1;
      continue;
    }

    if left != right {
      return false
    }

    l += 1;
    r -= 1;   
  }
  return true;
}
```