fn main() {
    println!("Hello, world!");
}

// TRAITS

// trait Describe {
//  fn describe_user(&self) -> String;
// }

// struct Person {
//     name: String,
//     age :i32,
// }

// impl Describe for Person {
//     fn describe(&self) -> String {
//         format!("{} is {} years old", self.name, self.age)
//     }
// }

// fn main (){
//     let p = Person {
//         name: String::from("John"),
//         age: 30,
//     };
//     println!("{}", p.describe());
// }

// TRAITS Multiply
// create a trait multiply
// the implementation should multiply the number in the vector
// return the result of the implementation

// trait Multiply {
//     fn multiply(&self) -> i32;
// }

// impl Multiply for Vec<i32>{
//     fn multiply(&self) -> i32 {
//         self.iter().product()
//     }
  
// }

// fn print_ultiply_result<K :Multiply>(items: K){

//     println!("The result is {}", items.multiply());
// }

// fn main () {
//     let numbers: Vec<i32> = vec![1, 2, 3, 4, 6];
//     print_ultiply_result(numbers);
// }

// create a struct called news
// struct should have three properties name,headlines and a number 
// create a trait that has functions
//  1 Funtion has default implemetation
//  2 function does not have default implementaion, create a summary for news 
//  have an implementation of the trait for new struct
// call the two trait functions form main



// struct News {
//     headlines: String,
//     name:String,
//     number:i32,
// }

// trait SummaryNews{
//     fn summary(&self) -> News;
// }

// impl SummaryNews for News{
//     fn summary(&self) -> News {
//         News {
//             headlines: self.headlines,
//             name: self.name,
//             number: self.number,
//         }
//     }
// }

// fn print_news()


//  program to calculates area for different shares
// calculate the area of the  area circle and rectangle
// ps : interrested in use of traits


