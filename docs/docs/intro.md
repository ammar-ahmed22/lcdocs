---
sidebar_position: 1
slug: /
---

# Introduction

Here you will find all of my notes as well as code for LeetCode problems that I have solved.

## Why did I create this?
I had two issues while studying LeetCode style problems:
1. Lack of autocomplete and code snippets in the LeetCode editors
2. Making notes about how I solved the problems

To tackle these issues, I created this CLI (command-line interface) project. The CLI does two things: 
1. Create a dedicated environment for running and testing the problems in Rust
2. Create a docs page in this website where I can write down my notes.

## CLI Usage
For the sake of example, let's say I was working on solving the infamous "Two Sum" problem. 

### Create the problem environment and docs
First, I would create the Rust problem environment and the docs page
```bash
lcdocs create "Two Sum" --difficulty easy
```

### Solve and test the problem
Now, I have a new folder inside `./easy` called `two_sum`, that has the following in it's `main.rs`

```rust
// main.rs
mod test_case;
use test_case::TestCase;

struct Solution {}

impl Solution {
  pub fn two_sum(nums: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    for value in nums.iter() {
      if value > &max {
        max = *value;
      }
    }

    return max;
  }
}



fn main() {
  let test_cases: Vec<TestCase<Vec<i32>, i32>> = vec![
    TestCase::new(vec![1, 2, 3, 8], 8),
    TestCase::new(vec![-1, -100, -2, 4], 4),
    TestCase::new(vec![-1, -100, -1000], -10)
  ];

  for (i, test_case) in test_cases.iter().enumerate() {
    test_case.run_and_log(|v| Solution::max_array(v), (i + 1).to_string());
  }
}
```

I can edit the `Solution` struct to house the actual problem and it's solution:

```rust
impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
          return vec![idx, i as i32];
        }
      }
    }

    return vec![0, 0]
  }
}
```
I also need to edit the testcases:
```rust

```

<!-- ## Getting Started

Get started by **creating a new site**.

Or **try Docusaurus immediately** with **[docusaurus.new](https://docusaurus.new)**.

### What you'll need

- [Node.js](https://nodejs.org/en/download/) version 18.0 or above:
  - When installing Node.js, you are recommended to check all checkboxes related to dependencies.

## Generate a new site

Generate a new Docusaurus site using the **classic template**.

The classic template will automatically be added to your project after you run the command:

```bash
npm init docusaurus@latest my-website classic
```

You can type this command into Command Prompt, Powershell, Terminal, or any other integrated terminal of your code editor.

The command also installs all necessary dependencies you need to run Docusaurus.

## Start your site

Run the development server:

```bash
cd my-website
npm run start
```

The `cd` command changes the directory you're working with. In order to work with your newly created Docusaurus site, you'll need to navigate the terminal there.

The `npm run start` command builds your website locally and serves it through a development server, ready for you to view at http://localhost:3000/.

Open `docs/intro.md` (this page) and edit some lines: the site **reloads automatically** and displays your changes. -->
