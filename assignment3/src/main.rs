

  /* Task 1

    Print out an items in reverse from 50 to 1 (hint: range operator)
    Task 2
    Create a program that takes in an input on the terminal(hint: input should be an integer, print out from 0 to the input(see a loop, for in and while loop))
    Task 3
    Create a program that takes in an input from the terminal and subtract, addition, multiply and division (any number)

     */
    use std::io;

fn main() {
    // Range operator Task 1
    // for x in (1..51).rev(){
    //     println!("The range is {}", x);
    // }

    let mut avocado_number = String::new();
        println!("Enter the number of avocados you want to buy: ");

    io::stdin().read_line(&mut avocado_number).expect("Failed to read line");

    // let change_number = avocado_number.trim().parse().unwrap();

    // task2(change_number);

    // Calling task3
    let mut number_1 = String ::new();
    let mut number_2 = String ::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut number_1).expect("Failed to read line");

    println!("Enter the second number: ");
    io::stdin().read_line(&mut number_2).expect("Failed to read line");

    let change_number1 = number_1.trim().parse().unwrap();
    let change_number2 = number_2.trim().parse().unwrap();

    task3(change_number1, change_number2);

}

// Task2

fn task2(input:i32){
    for oranges in 0..input{
        println!("The range is {}", oranges);
    }

    // println!("The range between is {}", oranges)

}

// Task3
fn task3(number:i32, number_2:i32){

    // summation
    let mut sum:i32 = number + number_2;
    println!("The sum is {}", sum);

    // subtraction
    let mut subtraction:i32 = number - number_2;
    println!("The subtraction is {}", subtraction);

    // multiplication
    let mut multiplication:i32 = number * number_2;
    println!("The multiplication is {}", multiplication);

    // division
    let mut division:i32 = number / number_2;
    println!("The division is {}", division);

}
