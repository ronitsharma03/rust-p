use std::io;

fn main(){
    println!("Enter a number : ");
    let mut number = String::new();
     io::stdin()
     .read_line(&mut number)
     .expect("Failed to read the number");


    let number: u32 = match number.trim().parse() {
        Ok (num) => num,
        Err(_) => {
            println!("Please enter a valid number ");
            return;
        }
    };

    if number % 2 == 0 {
        println!("{} is even!", number);
    } else{
        println!("{} is odd !", number);
    }
}