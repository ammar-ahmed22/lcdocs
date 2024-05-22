#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::{ HashMap, HashSet };
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: Vec<Vec<char>>,
    expected: bool,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let board = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), is_valid_sudoku(board.clone()))

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
        input: vec![
         vec!['5','3','.','.','7','.','.','.','.']
        ,vec!['6','.','.','1','9','5','.','.','.']
        ,vec!['.','9','8','.','.','.','.','6','.']
        ,vec!['8','.','.','.','6','.','.','.','3']
        ,vec!['4','.','.','8','.','3','.','.','1']
        ,vec!['7','.','.','.','2','.','.','.','6']
        ,vec!['.','6','.','.','.','.','2','8','.']
        ,vec!['.','.','.','4','1','9','.','.','5']
        ,vec!['.','.','.','.','8','.','.','7','9']
        ],
        expected: true
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: vec![
         vec!['8','3','.','.','7','.','.','.','.']
        ,vec!['6','.','.','1','9','5','.','.','.']
        ,vec!['.','9','8','.','.','.','.','6','.']
        ,vec!['8','.','.','.','6','.','.','.','3']
        ,vec!['4','.','.','8','.','3','.','.','1']
        ,vec!['7','.','.','.','2','.','.','.','6']
        ,vec!['.','6','.','.','.','.','2','8','.']
        ,vec!['.','.','.','4','1','9','.','.','5']
        ,vec!['.','.','.','.','8','.','.','7','9']
        ],
        expected: false
      }
    );
    Mutex::new(m)
  };
}

// Edit this function for the problem
fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
  let mut set: HashSet<char> = HashSet::new();
  
  // Checking rows
  for row in board.iter() {
    set.clear();
    for char in row.iter() {
      if *char != '.' {
        if set.contains(char) {
          return false;
        } else {
          set.insert(*char);
        } 
      }
    }
  }

  // Checking cols
  for i in 0..9 {
    set.clear();
    for j in 0..9 {
      let char = board[j][i];
      if char != '.' {
        if set.contains(&char) {
          return false;
        } else {
          set.insert(char);
        }
      }
    }
  }

  // Checking sub-boxes
  for box_row in 0..3 {
    for box_col in 0..3 {
      set.clear();
      for row in 0..3 {
        for col in 0..3 {
          let char = board[box_row * 3 + row][box_col * 3 + col];
          if char != '.' {
            if set.contains(&char) {
              return false;
            } else {
              set.insert(char);
            }
          }
        }
      }
    }
  }

  return true;
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
        let board = &test.input;
        let expected = &test.expected;
        let result = is_valid_sudoku(board.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
