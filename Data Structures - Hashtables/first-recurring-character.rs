/*
  Rust Type System: Rust requires explicit types. Here, I've used a slice 
  (&[i32]) to pass the list to the functions. This avoids ownership issues 
  and allows for efficient passing of data without copying.

  HashMap Usage: Rust's HashMap is used in the second function similar to a 
  Python dictionary. I've stored a bool as the value just to signify presence.

  Error Handling: Rust code does not inherently require error handling for 
  these operations, as they deal with standard types and control structures.

  Looping Constructs: Rust uses a range-based looping construct that avoids 
  directly accessing the length of the list.

  Return Values: Rust functions specify the type of the return value in the 
  function signature. Here, i32 is specified, which is an integer type.
*/
use std::collections::HashMap;

fn find_first_duplicate(my_list: &[i32]) -> i32 {
    for i in 0..my_list.len() {
        for j in (i + 1)..my_list.len() {
            if my_list[i] == my_list[j] {
                return my_list[i];
            }
        }
    }
    0
}

fn find_first_duplicate_with_hashtable(my_list: &[i32]) -> i32 {
    let mut my_dict = HashMap::new();
    for &item in my_list {
        if my_dict.contains_key(&item) {
            return item;
        } else {
            my_dict.insert(item, true);
        }
    }
    0
}

fn main() {
    let my_list = vec![2, 1, 1, 2, 3, 4, 5];
    let x = find_first_duplicate_with_hashtable(&my_list);
    println!("{}", x);
}
