---
sidebar_position: 9
---

# Evaluate Reverse Polish Notation
You are given an array of strings `tokens` that represents an arithmetic expression in a [Reverse Polish Notation](http://en.wikipedia.org/wiki/Reverse_Polish_notation).

Evaluate the expression. Return an integer that represents the value of the expression.

Note that:
- The valid operators are `'+'`, `'-'`, `'*'`, and `'/'`.
- Each operand may be an integer or another expression.
- The division between two integers always truncates toward zero.
- There will not be any division by zero.
- The input represents a valid arithmetic expression in a reverse polish notation.
- The answer and all the intermediate calculations can be represented in a 32-bit integer.

## Examples
### Example 1
> `tokens = ["2","1","+","3","*"]` <br />
> `Output: 9` <br />
> `Explanation: ((2 + 1) * 3) = 9`

### Example 2
> `tokens = ["4","13","5","/","+"]` <br />
> `Output: 6` <br />
> `Explanation: (4 + (13 / 5)) = 6`

## Notes
### Conceptual Idea
- Use a stack
- Create a function that can evaluate the 2 numbers with the 4 operands where order matters
  + e.g `fn eval(int a, int b, char operand) -> int`
  + running this with `'-'` should yield `a - b`  
- Iterate over the tokens
  + Whenever it is a number, push to the stack
  + When it is an operand
    - Pop the last 2 elements from the stack (there should always be atleast 2)
    - `let a: int = stack.pop()`
    - `let b: int = stack.pop()`
    - Evaluate the 2 values with the `eval` function and push the result to the stack
- Return the last element in the stack

### Complexity
- Time: O(n)
- Space: O(n)

## Solution
### Rust
```rust
fn eval(a: i32, b: i32, op: &String) -> i32 {
  match op.as_str() {
    "+" => {
      return a + b;
    },
    "-" => {
      return a - b;
    },
    "/" => {
      return a / b;
    },
    "*" => {
      return a * b;
    },
    _ => {
      return 0;
    }
  }
}


fn eval_rpn(tokens: Vec<String>) -> i32 {
  let mut stack: Vec<i32> = Vec::new();
  for char in tokens.iter() {
    match char.parse::<i32>() {
      Ok(number) => {
        stack.push(number);
      },
      Err(_) => {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(eval(a, b, char));
      } 
    }
  }
  return stack[stack.len() - 1];
}
```