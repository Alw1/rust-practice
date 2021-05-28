use std::io;

fn main() {
    
    println!("Enter what number in the fibbonaci sequence to find:  ");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: u32 = user_input
        .trim()
        .parse()
        .expect("Please type a number!");

    fn fibonnaci(x: u32) -> u32 // finds the nth fibbonaci number
    {
        let mut array: [u32;3] = [0,1,0];
        let mut count: u32 = 0;
        while count<x
        {
            array[0] = array[1];
            array[1] = array[2];
            array[2] = array[0] + array[1];
            count += 1;
        }
        return array[2];
    }

    println!("The {} number in the fibbonaci sequence is {}" ,user_input, fibonnaci(user_input));
}