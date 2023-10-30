//Importing the input and output library
use std::io;

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


        //Here goes the logic for the guess game
    println!("Your guess number is {guess}");
    if guess.trim() == "1"{
        println!("Congratulations{happy_face}...You are right");
    }
    else{
        println!("Sorry try again{sad_face}...You lost");
    }
}