#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: (String, String),
    expected: bool,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let (s, t) = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), is_anagram(s.to_string(), t.to_string()))

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
        input: (String::from("anagram"), String::from("nagaram")),
        expected: true
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: (String::from("rat"), String::from("car")),
        expected: false
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: (String::from("gainly"), String::from("laying")),
        expected: true
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn is_anagram(s: String, t: String) -> bool {
  let mut freq: [i32; 26] = [0; 26];
  
  // Base case
  if s.len() != t.len() {
    return false;
  }

  // Increment for `s`
  for c in s.chars() {
    let i = c as usize;
    freq[i - 97] += 1;
  }

  // Decrement for `t`
  for c in t.chars() {
    let i = c as usize;
    freq[i - 97] -= 1;
  }

  // Check freqency is zero
  for f in freq.iter() {
    if *f != 0 {
      return false;
    }
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
        let (s, t) = &test.input;
        let expected = &test.expected;
        let result = is_anagram(s.to_string(), t.to_string());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
