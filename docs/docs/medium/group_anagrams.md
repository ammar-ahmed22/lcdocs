---
sidebar_position: 1
---

# Group Anagrams
Given an array of strings `strs`, group the **anagrams** together. You can return the answer in any order.

An **Anagram** is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

## Constraints
- `1 <= strs.length <= 104`
- `0 <= strs[i].length <= 100`
- `strs[i]` consists of lowercase English letters.

## Examples
### Example 1
> `Input: strs = ["eat","tea","tan","ate","nat","bat"]` <br />
> `Output: [["bat"],["nat","tan"],["ate","eat","tea"]]`

### Example 2
> `Input: strs = [""]` <br />
> `Output: [[""]]`

### Example 3
> `Input: strs = ["a"]` <br />
> `Output: [["a"]]`

## Notes
### Conceptual Idea
- Use similar approach to [Valid Anagram](../easy/valid_anagram.md)
- Count the frequency of each letter in each string
  + The frequency array is used as a key in a hashmap that stores the array of other anagram

Implementation:
- Initialize a hashmap with the key being a length 26 array of integers and the value being a vector of strings
- Iterate over the `strs`
  + On each iteration, initialize a length 26 array of integers initialized to zero (`freq`)
  + Iterate over each character in the current string, increment the corresponding index in the array (`a -> 0, b -> 1, etc.`)
  + Check if the hashmap contains the `freq` array as a key (same `freq` means anagram)
    - If it does, push the current string to the corresponding string vector
    - If it doesn't, add the `freq` array with a vector containing the current string to the hashmap
- Iterate over all the keys in the hashmap and push the values to a result vector and return it

### Complexity
#### Time
- Let's say `strs` is length `n` and the longest `strs[i]` has length `m`
- Iterate over the `strs` array once -> O(n)
- Inside the `strs` iterating, we iterate over each string once -> O(m)
- Results in O(n * m)

#### Space
- In the worst case, we add `n` entries to the hashmap -> O(n)

## Solution
### Rust
```rust
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
  let mut hash: HashMap<[i32; 26], Vec<String>> = HashMap::new();
  for s in strs.iter() {
    let mut freq: [i32; 26] = [0; 26];
    for c in s.chars() {
      let i = c as usize;
      freq[i - 97] += 1;
    }
    let updated = match hash.get(&freq) {
      Some(vec) => {
        let mut vec = vec.clone();
        vec.push(s.to_string());
        vec
      },
      None => {
        vec![s.to_string()]
      }
    };
    hash.insert(freq, updated);
  }
  let mut result: Vec<Vec<String>> = vec![];
  for (_, value) in hash.iter() {
    result.push(value.clone());
  }

  return result;        
}
```