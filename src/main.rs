use std::io; // for input and output
use rand::Rng; // to generate a random number
use std::cmp::Ordering; // for comparing numbers

fn main() {
    println!("Welcome to the Guessing Game!");

    // generates a number from 1 to 10 (we will not change this value so we leave it as immutable variable)
    let secret_number = rand::thread_rng().gen_range(1..11);


    loop{
        println!("Please input a guess from 1 to 10!");

        /**for now the string is empty */
        let mut guess = String::new();

        /**we pass the reference to the readline since it will modify the value of guess
         *  expect is for failure */
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        /** convert to a integer and check for errors if it isn't */

        /**This is known as "shadowing." It allows you to reuse the variable name 
         * for a new value with potentially a different type. 
         * Shadowing is useful because it lets you transform variables step by step, starting from a raw form 
         * and refining them with transformations and checks without needing to come up with new variable names at each step. */

        let guess: u8 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>{
                println!("Please type a number");
                continue;
            },
        };


        println!("You guessed the number {}", guess);


        /**because cmp takes in type T 
         * It provides a consistent interface for all types, not just those that are Copy. 
         * For non-Copy types, passing by reference is necessary to avoid ownership transfer and potential cloning.
         */


        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal =>{
                println!("You guessed right!");
                break;
            }
        }


    }


}
