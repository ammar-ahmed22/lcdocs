#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: Vec<i32>,
    expected: i32,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let height = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), max_area(height.clone()))

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            // assert_eq_unordered!(expected.clone(), max_area(height.clone()));
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
        input: vec![1,8,6,2,5,4,8,3,7],
        expected: 49
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: vec![1, 1],
        expected: 1
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

fn calc_area(l_ptr: usize, l_val: i32, r_ptr: usize, r_val: i32) -> i32 {
  let height = std::cmp::min(l_val, r_val);
  let width = (r_ptr - l_ptr) as i32;
  return width * height;
}

// Edit this function for the problem
fn max_area(height: Vec<i32>) -> i32 {
  let mut max = 0;
  let mut l = 0 as usize;
  let mut r = height.len() - 1;

  while l < r {
    let l_val = height[l];
    let r_val = height[r];
    max = std::cmp::max(max, calc_area(l, l_val, r, r_val));
    if l_val == r_val {
      r -= 1;
      l += 1;
    } else if l_val < r_val {
      l += 1;
    } else {
      r -= 1;
    }
   }
  return max;
}

// Add your tests here (use the same id as the key for the TEST_DATA)
run_test! {
  example_1,
  example_2,
  // example_3,
}

fn main() {
    // Here we can run the code with log statements shown for debugging
    let test_data = &*TEST_DATA.lock().unwrap();
    for (name, test) in test_data.into_iter() {
        let height = &test.input;
        let expected = &test.expected;
        let result = max_area(height.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
