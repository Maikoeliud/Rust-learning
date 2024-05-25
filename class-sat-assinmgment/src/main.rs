

// /You have been give a string
// The string the lazy fox jumped the bridge
// find each occurence of each character, the number of times it occurs
// Input a string
// Count occurence of the words

// fn main() {
//     let sentence_string = String::from("The lazy fox jumped the bridge");
//     let character_string = sentence_string.split().unwrap;

//     let mut numberString = find_character_occurence(sentence_string.split() ,character_string);

//     println!("{}",numberString)

// }

// fn find_character_occurence(sentence_string:String, character:char) -> i32 {

// let mut counter = 0;

// for character in sentence_string.chars() {
//     if character == character {
//         counter += 1;
//     }
// }
// return counter;
// }


// fn main() {
//     let sentence = String::from("The lazy fox jumped over the bridge lazy");
//     let mut character_count = find_character_occurrences(sentence);

//     for (character, count) in &character_count {
//         println!("'{}' appears {} times", character, count);
//     }
// }

// fn find_character_occurrences(sentence:String) -> std::collections::HashMap<char, i32> {
//     let mut counter = std::collections::HashMap::new();
//     for character in sentence.chars() {
//         if !character.is_whitespace() {  // To ignore spaces if necessary
//             *counter.entry(character).or_insert(0) += 1;
//         }
//     }
//     counter
// }

// Counting words

// fn main(){
//     let sentence = ("The lazy fox jumped over the bridge lazy");

//     let words:Vec<&str> = sentence.split_whitespace().collect();

//     println!("The sentence has {}",words.len());


// }

// Create a program that useds the rand Create that create 10 random numbers and prints those numbers in descending order

// std::rand::Rng;
// fn main() {
//  use rand::Rng;

//  let mut numbers:Vec<i32> = Vec::new();

//  for _x in 0..10 {
//      let mut random_numbers = rand::thread_rng();
//      let number:i32 = random_numbers.gen_range(1..101);
//      numbers.push(number);
//  }
//  numbers.sort_by(|a, b| b.cmp(a));

//  for number in numbers {
//      println!("The number is {:?}",number);
//  }

// //  println!("The numbers are {}",numbers);
// }

// Write a Rust function that takes a vector of tuples (i32, i32)
//  and returns the sum of all the first elements if the second
//  elements are all even, the sum of all the second elements if
//  the first elements are all odd, and 0 otherwise.

fn main(){
    let tuple_one: Vec<(i32 , i32)> = vec![(1, 2), (2, 4), (3, 4), (4, 6)];
    let mut sum_of_elements = sum_of_elements(tuple_one);
    println!("The sum of the elements is {}",sum_of_elements);
}

fn sum_of_elements(tuple_numbers:Vec<(i32 , i32)>) -> Option<i32>{

    let all_second_elements_even_1 = true
    let all_first_elements_odd_1 = true

    for (item_1, item_2) in tuple_numbers.iter() {
        if item_2 % 2 == 0 {
            let sum += item_1;
        } 
        else if item_1 % 2 != 0 {
            let sum += item_2;
        }
        
        else {
             return Some((0));
        }
    }

    
   
}