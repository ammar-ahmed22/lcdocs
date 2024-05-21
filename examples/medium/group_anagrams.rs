#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: Vec<String>,
    expected: Vec<Vec<String>>,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let strs = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            // assert_eq!(expected.clone(), two_sum(nums.clone(), *target))

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            assert_eq_unordered!(expected.clone(), group_anagrams(strs.clone()));
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
        input: vec!["eat","tea","tan","ate","nat","bat"].iter().map(|s| s.to_string()).collect(),
        expected: vec![vec!["bat"], vec!["nat","tan"], vec!["ate","eat","tea"]].iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect()
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: vec![""].iter().map(|s| s.to_string()).collect(),
        expected: vec![vec![""]].iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect()
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: vec!["a"].iter().map(|s| s.to_string()).collect(),
        expected: vec![["a"]].iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect()
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
  let mut hash: HashMap<[i32; 26], Vec<String>> = HashMap::new();
  for s in strs.iter() {
    let mut freq: [i32; 26] = [0; 26];
    for c in s.chars() {
      let i = c as usize;
      freq[i - 97] += 1;
    }
    let updated = match hash.get(&freq) {
      Some(vec) => {
        let mut vec = vec.clone();
        vec.push(s.to_string());
        vec
      },
      None => {
        vec![s.to_string()]
      }
    };
    hash.insert(freq, updated);
  }
  let mut result: Vec<Vec<String>> = vec![];
  for (_, value) in hash.iter() {
    result.push(value.clone());
  }

  return result;        
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
        let strs = &test.input;
        let expected = &test.expected;
        let result = group_anagrams(strs.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
