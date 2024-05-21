<div align="center">
  <h1>LeetCode Docs</h1>
  <p>
    A Rust CLI to help me with creating testing environments for LeetCode problems as well as a documentation website for the solutions.
  </p>
</div>

## Overview
While studying LeetCode problems, I had two issues:
1. Lack of autocomplete and code snippets in the LeetCode editors
2. Making notes about how I solved the problems

In order to solve this, I created this CLI which does two things:
1. Create a dedicated environment for running and testing the problems in Rust
2. Create a docs page on a custom [Docusaurus](https://google.ca) website

The CLI also helps me with running and testing the problem in the Rust environment.

## CLI Usage
We'll use the infamous [Valid Parentheses](https://leetcode.com/problems/valid-parentheses/description/) problem to showcase how I would use the CLI.

### Creating the problem
Run the `create` command:
```bash
lcdocs create "Valid Parentheses" -d easy
```
The `create` command takes 2 arguments:
1. The name of the problem
2. The difficulty of the problem (used for organization in the docs page)

The format of the command is as follows: `lcdocs create <PROBLEM> --difficulty | -d <DIFFICULTY>`

Valid options for `<DIFFICULTY>` are:
- `easy`
- `medium`
- `hard`

This command does two things:
1. Creates a file in the 


