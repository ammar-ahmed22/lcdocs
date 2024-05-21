---
sidebar_position: 3
---

# Problem Name
Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`, and `false` otherwise.

An **Anagram** is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

## Constraints
- `1 <= s.length, t.length <= 5 * 104`
- `s` and `t` consist of lowercase English letters.


## Examples
### Example 1
> `Input: s = "anagram", t = "nagaram"` <br />
> `Output: true`

### Example 2
> `Input: s = "rat", t = "car"` <br />
> `Output: false`

## Notes
### Conceptual Idea
- As the characters of `s` and `t` are constrained to lowercase English letters, we know there are only 26 possibilities for each letter
- Check if `s` and `t` are the same length -> return false if not 
- Create an array of 26 integer values initialized to 0 (used to count the frequency of each letter in a string)
- Iterate over `s`
  + Increment the array value for each letter
    - i.e. `a` -> `0th` index in the array, `b` -> `1st` index in the array, etc.
- Iterate over `t`
  + Decremenmt the array value for each letter
- Check if the count array contains all zeros -> return true
- Otherwise, return false
- An anagram will have the same frequency of letters, so, we increment for 1 and decrement for the other. 
  + Anagram will have result in all zeroes

### Complexity
#### Time
- Let's say `s` has length `n` and `t` has length `m`
- First, we iterate over `s` -> O(n)
- Second, we iterate voer `t` -> O(m)
- Last, we iterate over the count array -> O(26) (const.)
- Results in O(n + m + 26) -> O(n + m)

#### Space
- Only new space we create is the 26 length array for counting -> O(26) -> O(1)

## Solution
### Rust
```rust
fn is_anagram(s: String, t: String) -> bool {
  let mut freq: [i32; 26] = [0; 26];
  
  // Base case
  if s.len() != t.len() {
    return false;
  }

  // Increment for `s`
  for c in s.chars() {
    let i = c as usize;
    freq[i - 97] += 1;
  }

  // Decrement for `t`
  for c in t.chars() {
    let i = c as usize;
    freq[i - 97] -= 1;
  }

  // Check freqency is zero
  for f in freq.iter() {
    if *f != 0 {
      return false;
    }
  }

  return true;
}
```