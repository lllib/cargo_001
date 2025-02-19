use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    println!("Guess the number!");

    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..=99);
    let mut guess = String::new();


    loop{
            guess.clear();
        
            io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // 检查是否要退出
        if guess.trim().eq("quit") {
            println!("Thanks for playing!");
            break;
        } 
        //let guess_u32:u32 = guess.trim().parse().expect("Please type a number!");
        let guess_u32:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess_u32.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    
   
    
        println!("you guessed:{}...{}",guess,secret_number);
    }

}
