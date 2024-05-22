#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: Vec<i32>,
    expected: Vec<i32>,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let nums = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), product_except_self(nums.clone()))

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
        input: vec![1,2,3,4],
        expected: vec![24,12,8,6]
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: vec![-1,1,0,-3,3],
        expected: vec![0,0,9,0,0]
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
  let n = nums.len();
  let mut prefix: Vec<i32> = vec![1; n + 1];
  let mut suffix: Vec<i32> = vec![1; n + 1];
  for (i, num) in nums.iter().enumerate() {
    prefix[i + 1] = prefix[i] * num;
  }
  for (i, num) in nums.iter().rev().enumerate() {
    suffix[n - i - 1] = suffix[n - i] * num;
  }
  let mut result: Vec<i32> = vec![];
  for i in 1..suffix.len() {
    result.push(prefix[i - 1] * suffix[i]);
  }
  return result;
}

// Add your tests here (use the same id as the key for the TEST_DATA)
run_test! {
  example_1,
  example_2,
}

fn main() {
    // Here we can run the code with log statements shown for debugging
    let test_data = &*TEST_DATA.lock().unwrap();
    for (name, test) in test_data.into_iter() {
        let nums = &test.input;
        let expected = &test.expected;
        let result = product_except_self(nums.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
