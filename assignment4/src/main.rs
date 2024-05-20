// 1. create an externnal module
// a . file module
// b. directory module
// file module -> function takes in a slices and returns the second item in the slice
// directory module -> println functions
// call the file module function and directory function from main rs

mod countries;

use countries::african_countries::east_african;
use countries::asian_countries::central_countries;

// use asian_countries::central_asian_countries;

mod Fruits;

use Fruits::{apples, oranges, avocadoes};

fn main() {
    // countries::centra_countries::centra_countries();
    // println!("Hello, world!");

    // fibonacci();
    // apples();
    // oranges();
    // avocadoes();
    east_african();
    central_countries();
}
fn fibonacci(position:i32) {

}

