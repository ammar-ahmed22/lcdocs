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
1. Creates a file in the `examples` directory of the Rust project in the correct directory `easy | medium | hard`, called `valid_parentheses.rs` populated with a template file showcasing how to test and run the [Two Sum](https://leetcode.com/problems/two-sum) problem
2. Creates a file in the `docs` directory called `valid_parentheses.md` with some template markdown for my notes on the problem.

### Running and testing the problem
In order to run the problem, we'll need to edit the newly created file `examples/easy/valid_parentheses.rs` to house the actual problem in question. 

I won't show the entire file, as it is a little long, however, you can check out what the file looks like before any edits [here](./examples/_template.rs)

To start, we'll change the `two_sum` problem to `is_valid` following  the [Valid Parentheses]() problem:
```rust
// valid_parentheses.rs
// ...
// Edit this function for the problem
fn is_valid(s: String) -> bool {
  return false // to remove compilation errors
}
```

Next, we'll edit the `TestData` type to include the correct input and output types for the problem:
```rust
// valid_parentheses.rs
// ...
#[derive(Debug)]
struct TestData {
    input: String,
    expected: bool,
}
```

From this, we'll need to edit the `TEST_DATA` HashMap which houses all of the tests we want to run:
```rust
// valid_parentheses.rs
// ...
lazy_static! {
  static ref TEST_DATA: Mutex<HashMap<&'static str, TestData>> = {
    let mut m = HashMap::new();
    m.insert(
      "example_1",
      TestData {
        input: String::from("()"),
        expected: true
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: String::from("()[]{}"),
        expected: true
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: String::from("(]"),
        expected: false
      }
    );

    Mutex::new(m)
  };
}
```

From this, we need to edit the `main` function as well as the `run_test` macro to use the `is_valid` function with the correct inputs:

Here is the edited `run_test` macro:
```rust
// valid_parentheses.rs
// ...
macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let s = &test.input; // Extracting the string input
            let expected = &test.expected; // Extracting the bool expected output

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]

            // Checking for equality of the expected output and the actual output
            assert_eq!(expected.clone(), is_valid(s.to_string()));

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            // assert_eq_unordered!(expected.clone(), is_valid(s.to_string()));
          }
        )*
    };
}
``` 

Here is the edited `main` function:
```rust
fn main() {
    // Here we can run the code with log statements shown for debugging
    let test_data = &*TEST_DATA.lock().unwrap();
    for (name, test) in test_data.into_iter() {
        let s = &test.input;
        let expected = &test.expected;
        let result = is_valid(s.to_string());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
```

Now, we are ready to run/test the problem. You can check out the solution in [valid_parentheses.rs](./examples/easy/valid_parentheses.rs)

To run the problem (`main`), we can use:
```bash
lcdocs run valid_parentheses
```
which gives the following output:
```bash
[INFO]: Found 'easy' problem 'valid_parentheses'
[INFO]: Running 'valid_parentheses':
   Compiling lcdocs v0.1.0 (/Users/ammar/Documents/Projects/Rust/lcdocs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/examples/valid_parentheses`
Running 'example_1'
expected: true, received: true
Running 'example_3'
expected: false, received: false
Running 'example_2'
expected: true, received: true
[INFO]: Finished running 'valid_parentheses'
```

To test the problem, we can use:
```bash
lcdocs test valid_parentheses
```

which gives the following output:
```bash
[INFO]: Found 'easy' problem 'valid_parentheses'
[INFO]: Testing 'valid_parentheses'
   Compiling lcdocs v0.1.0 (/Users/ammar/Documents/Projects/Rust/lcdocs)
    Finished test [unoptimized + debuginfo] target(s) in 0.41s
     Running unittests examples/easy/valid_parentheses.rs (target/debug/examples/valid_parentheses-93a955236a87cf66)

running 3 tests
test example_1 ... ok
test example_2 ... ok
test example_3 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[INFO]: Finished testing 'valid_parentheses'
```

### Writing the docs
From this, I can edit the file in the `docs` folder called `valid_parentheses.md` with any notes I have in regards to solving the problem.

To run the docs website from the CLI, I can use the command:
```bash
lcdocs docs
```

### CLI Options
Running the command `lcdocs help` shows all the available options for the CLI:
```bash
A CLI to help me organize my leetcode solutions!

Usage: lcdocs <COMMAND>

Commands:
  create  Create a problem
  run     Run a problem
  remove  Remove a problem
  test    Test a problem
  docs    
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```


## CI/CD
Continuous deployment is setup with GitHub Actions to deploy the docs website whenever the `docs` folder changes!

## License
[MIT](./LICENSE)