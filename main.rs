use rand::Rng; 
// use to specify the range of the random number
use std::cmp::Ordering;
use std::io; // enum which is the result of 2 things being compared.
use colored::*; // will bring colors to the print statement.
fn main() {
    println!("Guess the number! ");
    let secret_number = rand::thread_rng().gen_range(1, 11);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your number: ");

    let mut guess: String = String::new(); // new reuturns an empty string.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess :u32 = match guess.trim().parse(){
        Ok(num ) => num,
        Err(_) => continue,//underscore is a catch whole value.
        }; // parse is a method that takes a string and tries to turn it into another type.

    println!("You guessed {}", guess);
    match guess.cmp(&secret_number) { // we can't compare a integer with the string.
    Ordering::Less => println!("{}", "TOO SMALL JUST LIKE YOUR PP!!!!" .red()),
    Ordering::Greater => println!("{}", "TOO BIG JUST LIKE AN ELEPHANT TUSK" .cyan()),
        Ordering::Equal => {

         println!("{}", "JUST PERFECT".green());
        break;
        }
    }
    }
}
