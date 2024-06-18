#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: (Vec<Vec<i32>>, i32),
    expected: bool,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let (matrix, target) = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), search_matrix(matrix.clone(), *target))

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
        input: (vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 3),
        expected: true
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: (vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 13),
        expected: false
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
   let m = matrix.len();
   let n = matrix[0].len();
   let mut top = 0 as usize;
   let mut bot = m - 1 as usize;

   while top < m && bot < m && top <= bot {
    let mid = (top + bot) / 2;
    if matrix[mid][0] <= target && matrix[mid][n - 1] >= target {
      if matrix[mid][0] == target || matrix[mid][n - 1] == target {
        return true;
      }
      let mut l = 0 as usize;
      let mut r = n - 1;
      while l < n && r < n && l <= r {
        let cen = (l + r) / 2;
        if matrix[mid][cen] == target {
          return true;
        } else if matrix[mid][cen] < target {
          l = cen + 1;
        } else {
          r = cen - 1;
        }
      }
      return false;
    } else if matrix[mid][0] < target {
      top = mid + 1;
    } else {
      bot = mid - 1;
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
        let (matrix, target) = &test.input;
        let expected = &test.expected;
        let result = search_matrix(matrix.clone(), *target);
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
