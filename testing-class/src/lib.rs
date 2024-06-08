// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }


// // you have been a trait that implements an area takes 
// // 
// // create a struct inpments the trait
// // create a test to check if the area is correct


// trait Area {
//     fn area(&self) -> i32;
// }

// struct Square {
//     width: i32,
//     height: i32,
// }

// impl Area for Square{
//     fn area(&self) -> i32 {
//         self.width * self.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

    
//      #[test]
//     fn area_works() {
//         let square = Square{
//             width: 5,
//             height:7
//         };
//         let result = square.area();
//         assert_eq!(result, 35);
//     }
// }
// create a closure that takes two numbers adds them and write the tests for those numbers


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn add_closure_works() {
// let add = |m: i32, n: i32| -> i32 {
//     m + n
// };
//         let result = add(1, 2);
//         assert_eq!(result, 3);
//     }

// }

// use num::
// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b == 0.0 {
//         let err = String::from("Cannot divide by zero");
//         return Err(err);
//     }
//     Ok(a / b)
// }

 // fn generic
 //num crate
 //

// use std::fmt::Display;
// use std::ops::Div;
// use num::Zero;

//  fn gen_divide<T>(a: T, b: T) -> Result<T, String> where 
//  : Zero + Div<Output = T> + Display + Copy + PartialOrd
//  {
//     if b == T::zero() {
//          let err = String::from("Cannot divide by zero");
//          return Err(err);
//      }
//      Ok(a / b)
//  }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn divide_works() {
//         let result = gen_divide(1.0, 1.0);
//         assert_eq!(result, Ok(1.0));
//     }

//     #[test]
//     fn divide_by_zero_fails() {
//         let result = gen_divide(1.0, 0.0);
//         assert_eq!(result, Err(String::from("Cannot divide by zero")));
//     }
// }

//Write a Rust function that takes a tuple (char, i32) and 
//returns "Vowel" if the first element is a vowel ('a', 'e', 'i', 'o', 'u') and "Consonant" otherwise.

// fn tuple_to_string(tuple_alphabet: (char, i32)) -> String {
//     let (character, number) = tuple_alphabet;

//     if character == 'a' || character == 'e' || character == 'i' || character == 'o' || character == 'u' {
//         return String::from("Vowel");
//     }
//     else {
//         return String::from("Consonant");
//     }

// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn tuple_alphabet_works() {
//         let tuple = ('T', 1);
//         let result = tuple_to_string(tuple);
//         assert_eq!(result, String::from("Consonant"));
//     }
// }
// Using Vector
// let a = vec![a, e, i, o, u]; <Vec>
// fn tuple_to_string(tuple_alphabet: (char, i32)) -> String {
//     let (character, number) = tuple_alphabet;

//     let my = vec!['a', 'e', 'i', 'o', 'u'];

//     let x = my.contains(&character);
    
//     match x {
//         true => String::from("Vowel"),
//         false => String::from("Consonant"),
//     }
//     // if character == ['a'] || character == ['e'] || character == ['i'] || character == ['o'] || character == ['u'] {
//     //     return String::from("Vowel");
//     // }
//     // else {
//     //     return String::from("Consonant");
//     // }

// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn tuple_alphabet_works() {
//         let tuple = ('T', 1);
//         let result = tuple_to_string(tuple);
//         assert_eq!(result, String::from("Consonant"));
//     }
// }


// fn tuple_to_string(tuple_alphabet: (char, i32)) -> String {
//     let (character, number) = tuple_alphabet;

//     match character {
//         'a' | 'e' | 'i' | 'o' | 'u' => String::from("Vowel"),
//         _ => String::from("Consonant"),
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn tuple_alphabet_works() {
//         let tuple = ('T', 1);
//         let result = tuple_to_string(tuple);
//         assert_eq!(result, String::from("Consonant"));
//     }
// }

// create a generic function that takes two paramter 
//and compares the two parameters returning the less/smaller of the two


//use std::cmp::Ordering;

// fn gen_par<T: Ord>(a: T, b: T) -> T {
//     if a < b {
//         a
//     } else {
//         b
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn gen_par_works() {
//         let result = gen_par(5, 1);
//         assert_eq!(result, 1);
//     }

//     #[test]
//     fn gen_par_works2() {
//         let result = gen_par('z', 'a');
//         assert_eq!(result, 'a');
//     }
// }

// using iterator adaptors

// Write a Rust function that takes a vector of booleans and returns "All true!"
//if all elements are true, "All false!" if all elements are false, and "Mixed!" otherwise.

fn vector_boolean(b: Vec<bool>) -> String {
    if b.iter().all(|&x| x) {
        "All true!".to_string()
    } else if b.iter().all(|&x| !x) {
        "All false!".to_string()
    } else {
        "Mixed!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_test() {
        let result = vector_boolean(vec![true, true, true]);
        assert_eq!(result, "All true!");

         let result = vector_boolean(vec![true, false, true]);
        assert_eq!(result, "Mixed!");
    
    }

 
}
