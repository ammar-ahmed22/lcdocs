#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: (Vec<i32>, i32),
    expected: i32
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let (nums, target) = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), search(nums.clone(), *target))

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            // assert_eq_unordered!(expected.clone(), search(nums.clone(), *target));
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
        input: (vec![-1,0,3,5,9,12], 9),
        expected: 4
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: (vec![-1,0,3,5,9,12], 2),
        expected: -1
      }
    );

    // m.insert(
    //   "example_3",
    //   TestData {
    //     input: (vec![3,3], 6),
    //     expected: vec![0, 1]
    //   }
    // );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0 as usize;
    let mut right = nums.len() - 1;
    while left < nums.len() && right < nums.len() && left <= right {
      let mid = (left + right) / 2;
      if nums[mid] == target {
        return mid as i32;
      } else if nums[mid] < target {
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    return -1;
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
        let (nums, target) = &test.input;
        let expected = &test.expected;
        let result = search(nums.clone(), *target);
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
