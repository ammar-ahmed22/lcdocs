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
            let s = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), is_palindrome(s.clone()));

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            // assert_eq_unordered!(expected.clone(), two_sum(nums.clone(), *target));
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
        input: String::from("A man, a plan, a canal: Panama"),
        expected: true
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: String::from("race a car"),
        expected: false
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: String::from(" "),
        expected: true
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
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
        let result = is_palindrome(s.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
