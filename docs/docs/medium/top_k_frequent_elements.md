---
sidebar_position: 2
---

# Top K Frequent Elements
Given an integer array `nums` and an integer `k`, return the _`k` most frequent elements_. You may return the answer in **any order**.

## Constraints
- `1 <= nums.length <= 105`
- `-104 <= nums[i] <= 104`
- `k` is in the range `[1, the number of unique elements in the array]`.
- It is **guaranteed** that the answer is **unique**.
- Your algorithm's time complexity must be better than `O(n log n)`, where `n` is the array's size.

## Examples
### Example 1
> `Input: nums = [1,1,1,2,2,3], k = 2` <br />
> `Output: [1,2]`

### Example 2
> `Input: nums = [1], k = 1` <br />
> `Output: [1]`


## Notes
### Conceptual Idea
- We can exploit the constraint that the answer is unique
  + This means that each repeated element will have a unique number of occurrences
  + We also know that the frequency of elements can't exceed the size of the input array
    - i.e. The maximum number of repetitions for a number is the size of the array (e.g. all 1's)
- Create a vector the same size as the input array, initialized to an empty Vector
- Count the frequency of numbers using a hashmap 
- Iterate over the hashmap
  + For each number, use the frequency as the index in the count vector and push the value to that vector
    - i.e. `count: Vec<Vec<i32>>`. 1 occurs once, 2 occurs once. `count[1].push(1), count[1].push(2)`
- Iterate over the array in reverse using a while loop and return `k` elements by stopping when result array has length `k`

### Complexity
#### Time
- We iterate over the numbers once to count the frequency -> O(n)
- Iterate over the hashmap (which can be length `n` at the worst case) -> O(n)
- Iterate over the array (which can be length `n` at the worst case) -> O(n)
- Results in O(n)

#### Space
- Hashmap (worst case length `n`) -> O(n)
- Array (worst case length `n`) -> O(n)
- Results in O(n)

## Solution
### Rust
```rust
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
  let n = nums.len();
  // Count frequency
  let mut freq: HashMap<i32, usize> = HashMap::new();
  for num in nums.iter() {
    let updated = match freq.get(num) {
      Some(frequency) => {
        *frequency + 1
      },
      None => {
        1
      }
    };
    freq.insert(*num, updated);
  }
  
  // Update the count array
  let mut count: Vec<Vec<i32>> = vec![vec![]; n + 1];
  for (key, val) in freq.iter() {
    count[*val].push(*key);
  }

  // Return the k most frequent elements
  let mut result: Vec<i32> = vec![];
  let mut i: usize = n;
  while result.len() < (k as usize) {
    for val in count[i].iter() {
      result.push(*val);
    }
    i -= 1;
  }

  return result;
}
```