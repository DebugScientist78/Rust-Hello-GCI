use std::io;

fn main() {
    //ask for user input
    println!("Input a number: ");

    //raw string input from user
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read Input");

    //get length of number
    //force cast as unsigned 32-bit
    let len: u32 = input
        .trim()
        .len() as u32;

    //convert the String to a 16-bit int
    let mut num: i16 = input
        .trim()
        .parse()
        .unwrap();

    //creating a temporary copy
    let expectedNum: i16 = num;

    //sum of each digit to len power
    let mut sum: i16 = 0;

    //r refers to a digit of the inputted number
    let mut r: i16 = 0;

    //iterate len times
    //calculates the sum of each digit to len power
    for x in 0..len {
        r = num%10;
        sum = sum + r.pow(len);
        num = num / 10;
    }
    //checks if the sum and initial number match
    if expectedNum == sum {
        println!("It is an Armstrong number!");
    } else {
        println!("It is not an Armstrong number!");
    }
}