use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number quall {}", secret_number);

    loop{
        println!("Pls, enter ur guess!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Wrong number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            

        println!("You guess: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("U won!");
                break;
            }
        }
    }
}
