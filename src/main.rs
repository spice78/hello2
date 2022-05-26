/*
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

        let guess : u16 = match guess.trim().parse() {
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
*/
/*===================================================*/
//Kortege und massive block
/*
fn foo(x:u8, y:u8, z:u8) -> [u8; 3]{
    return [x, y, z]
}

fn poo(x:u8, y:u8, z:u8){
    println!("This is poo x={}, y={}, z={}", x, y, z)
}

fn main() {
    //let tup = [77, 44, 53];
    poo(1, 2, 3);
    let pup = foo(4, 5, 6);
    println!("This is foo x={}, y={}, z={}", pup[0], pup[1], pup[2])
}
*/
/*=====================================================*/
//IF
/*
fn main(){
   let num = true;   
   let mun = if num{"true"}else{"false"};

   println!("{}", mun);
}
*/
/*====================================================*/
//LOOP
/*
fn main(){
    let mut count = 0;
    let mut num = 35;
    let res = loop {
        num -= 4;
        count += 1;
        if num < 4 {
            break count;
        }
    };
    println!("{}", res);
}
*/

fn main(){
    let mut count = 0;
    let mut num = 16;
    while num >= 4{
        num -= 4;
        count += 1;
    }    
    
    println!("{}", count);
}