---
sidebar_position: 4
---

# Problem Name
Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.

An input string is valid if:

1. Open brackets must be closed by the same type of brackets.
2. Open brackets must be closed in the correct order.
3. Every close bracket has a corresponding open bracket of the same type.

## Constraints
- 1 <= s.length <= 104
- `s` consists of parentheses only `'()[]{}'`.

## Examples
### Example 1
> `Input: s = "()"` <br />
> `Output: true`

### Example 2
> `Input: s = "()[]{}"` <br />
> `Output: true`

### Example 3
> `Input: s = "(]"` <br />
> `Output: false`

## Notes
### Conceptual Idea
- Can use a stack to track the closing braces
- Create a hashmap that stores the opening brace as the key and the closing brace as the value
- Create an empty stack
- Iterate over the characters in `s`
  + If the character is an opening brace (it can be found in the hashmap with a key lookup)
    - Push the closing brace to the stack (the value in the hashmap)
  + If the character is a closing brace (not found in the hashmap with key lookup)
    - Pop the last value in the stack
      + Check if the popped value is the same as the current character 
        - If they are different, it is the wrong closing brace because the last brace needs to close before any others -> return false
        - If they are the same, continue
- If we get to the end of the characters, return true if the stack is empty, otherwise false
  + If the stack has values, that means there are unclosed braces at the end of the string

### Complexity
#### Time
- As we only iterate over the string once, O(n)

#### Space
- The hashmap only has 3 entries -> O(3)
- The stack can have up to `n` entries -> O(n)
- Results in O(n)

## Solution
### Rust
```rust
fn main() {
  let mut map: HashMap<char, char> = HashMap::new();
  map.insert('{', '}');
  map.insert('(', ')');
  map.insert('[', ']');

  let mut stack: Vec<char> = vec![];

  for c in s.chars() {
    match map.get(&c) {
      Some(closing) => {
        // opening paren found
        // push closing to stack
        stack.push(*closing);
      },
      None => {
        // closing paren found
        // pop from the stack -> check if they are the same
        match stack.pop() {
          Some(top) => {
            if top != c {
              return false;
            }
          },
          None => {
            return false;
          }
        }
      }
    }
  }
  return stack.len() == 0
}
```