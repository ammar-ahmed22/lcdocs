#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: Vec<i32>,
    expected: bool,
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
            assert_eq!(*expected, contains_duplicate(nums.clone()))

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
        input: vec![1, 2, 3, 1],
        expected: true
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: vec![1, 2, 3, 4],
        expected: false
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
        expected: true
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn contains_duplicate(nums: Vec<i32>) -> bool {
  let mut hash: HashMap<i32, bool> = HashMap::new();
  for num in nums.iter() {
    match hash.get(num) {
      Some(_) => {
        return true;
      },
      None => {
        hash.insert(*num, true);
      }
    }
  }
  return false;        
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
        let nums = &test.input;
        let expected = &test.expected;
        let result = contains_duplicate(nums.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
