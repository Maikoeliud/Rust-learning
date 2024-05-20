
// using the knowledge of structs
// implement a game character: name , score, level

// struct Game {
//     name: String,
//     score: u32,
//     level: u32,
// }

// impl Game {
//     fn name(&self) -> &str {
//         &self.name
//     }
// }

// fn main() {
//     let game1 = Game {
//         name: String::from("Football"),
//         score: 3,
//         level: 2,
//     };

//     let game2 = Game {
//         name: String::from("Basketball"),
//         score: 3,
//         level: 1,
//     };

//     let result = game1.name();
//     println!("The name of the game is {}", result);
// }


// create a program
// takes two thing , one input, and another input from the terminal
// didvides firats input by the second input
// checks
use std::io;

fn main() {
    let mut first_number = String::new();
    let mut second_number = String::new();

    println!("Enter the first number: ");
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");

    println!("Enter the second number: ");
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    let first_number: i32 = first_number.trim().parse().unwrap();
    let second_number: i32 = second_number.trim().parse().unwrap();

    let divide = divide(first_number, second_number);

    println!("The division is {}", divide);
}

fn divide(first_input: i32, second_input: i32) -> i32 {
    first_input / second_input
}
