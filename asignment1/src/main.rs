

fn main() {
    // Numbers
//     let number:i8 = 45;
//     let number1:i16 = 4567;
//     let number2:i32= 34;
//     let number3:i64 = 4567;

//     println!("The number is {}",number);

//     // Characters
//     let character:&str = "welcome to web3";
//     println!("The character is {}",character);

//     let string_2 = String::from("hello world");
//     println!("The string is {}",string_2);

//     // Floating point numbers
//     let floating_number:f32 = 45.67;
//     println!("The floating number is {}",floating_number);

// //     // Functions
//  sum(45,56);
//  let summation = sum2(45,78);
//  println!("The summation is {}",summation);

// //  converting interger to a string
// let number_string:i32= 123;
// let mut number_string2 :String = number_string.to_string();

// number_string2.push_str(" people in the room");

// println!("The number string is {}",number_string2);

// let input_1= "31";

// let input_number:i32 = input_1.parse().expect("was expecting an interger");

// println!("The number is {}",input_number);

//    let (data, status,reason) = do_sum_maths(45,56);

// // enums
// const age:i32 =32;

// #[derive(Debug)]
// enum Status{
//     ADULT,
//     CHILD,
// }
// let user_status:Status = if age > 18{
//     Status::ADULT
// }else{
//     Status::CHILD
// };

// println!("The user status is {:?}",user_status);

// // greter than
// // equal to
// // less than

// let a:i32 = 45;
// let b:i32 = 56;

// if a > b{
//     println!("The number is greater than {}",b);
// }
//  if a < b{
//     println!("The number is less than {}",b);
// }
// if a == b {
//     println!("The number is equal to {}",b);
// }
// // loops
// let mut array = [1,2,3,4,5,6,7,8,9,10];
// let mut index = 6;

// while index != 0 {
//     println!("looping throuh indexes {index} item in the array is {}",array[index]);
//     index -= 1;
// }

// println!("LOOk its me!!!");

// packaging of files




}
 fn sum(number_1:i32,number2: i32){
        let number_3 = number_1 + number2;
        println!("The sum is {}",number_3);
    }
    fn sum2(number_1:i32,number2: i32)->i32{
        let number_3:i32 = number_1 + number2;
        return number_3;
    }

   





fn do_sum_maths(par_1:i32, par_2:i32) -> (i32, bool, String){
    return(0, true, "success".to_string());
}