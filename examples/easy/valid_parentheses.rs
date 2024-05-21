#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: String,
    expected: bool,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let s = &test.input; 
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), is_valid(s.to_string()));

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            // assert_eq_unordered!(expected.clone(), is_valid(s.to_string()));
          }
        )*
    };
}

lazy_static! {
  // Edit this with your tests cases
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

// Edit this function for the problem
fn is_valid(s: String) -> bool {
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

// Add your tests here (use the same id as the key for the TEST_DATA)
run_test! {
  example_1,
  example_2,
  example_3,
}

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
