fn main() {

    // Write a funtion that takes in an array of numbers and 
    // create a refrence to that array , loop through and adds them up and return sum
// let array_1 = [1,9,7,2,10];
// let total_summation = summation(&array_1);

//     println!("The total summation is {}", total_summation);

let mut value = String::from("what is this?");

    let mut called_function = string_funtion(& mut value);

    println!("{}", called_function);
}
// fn summation(array_1: &[u32]) -> u32 {
//     let mut sum: u32 = 0;
//     for i in array_1 {
//         sum = sum + i;
//     }
//     sum
// }

fn string_funtion(value:& mut String) -> & mut String {

 value.push_str("This is a string");

 value


}

// is that okay ? mic not working THANKS!!