//Before this code can run, you must have the Rand crate in your dependencies.
//You can manually add rand crate in dependencies in the Cargo.toml file

//Importing the input and output library
use std::io;
//Importing the RANDOM crate
use rand::Rng;
fn main(){
    println!("It's time for some guess game");
    println!("Enter your guess game number");

        //Declaring and assigning the variables
    let mut guess = String::new();

        //These are emoji unicodes
    let happy_face = '\u{1F600}';
    let sad_face = '\u{1F610}';


 
        //This is where the program takes and reads the input from the user   
        io::stdin()
        .read_line(&mut guess)

        
        //This code block handles any errors at runtime
        .expect("Error");

        //Assigning a range of numbers 1-100 inclusive and randomizing it 
        let secret_number = rand::thread_rng().gen_range(1..=100);

        //Parsing the users input into an integer for easy comparison
        let result = guess.trim().to_string();


        //Here goes the logic for the guess game
        match result.parse::<i32>(){
            Ok(user_number) =>{
                if user_number == secret_number{
                    println!("Congratulations{happy_face}...You are right")
                }
                else{
                    println!("Sorry try again{sad_face}...You lost")
                }
            }
            Err(_)=>{
                println!("failed to parse string to integer")
            }
        }

        println!("The secret number is: {secret_number}");



        //Here goes the logic for the guess game
    // println!("Your guess number is {guess}");
    // if guess.trim() == "1"{
    //     println!("Congratulations{happy_face}...You are right");
    // }
    // else{
    //     println!("Sorry try again{sad_face}...You lost");
    // }
}