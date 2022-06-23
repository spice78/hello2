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
/*
fn main(){
    let mut count = 0;
    let mut num = 64;
    let num2: i32 = num;
    while num >= 4{
        num -= 4;
        count += 1;        
    }

    println!("{}", count);
    fn_for(count, num2*count, num2);
}

fn fn_for(x:i32, mut z:i32, i:i32) {    
        
    for y in 0..x {
        println!("fn_for == {}",  z);
        z -= i;
    }    
}
*/
/*
fn main() {
    convert_fahr(315.0);
    convert_cels(315.0);
}

fn convert_fahr (x:f32) {
    let mut fahr:f32 = 0.0;

    while x >= fahr {        
        let celsius = (5.0/9.0)*(fahr-32.0);
        
        println!("cels - {}, fahr - {}", celsius, fahr);

        fahr += 45.0;
    }
}

fn convert_cels(x:f32) {
    let mut cels = 0.0;

    while x >= cels {
        let fahr = ((9.0/5.0) * cels) + 32.0;

        println!("cels - {}, fahr - {}", cels, fahr);
        
        cels += 45.0;
    }
}
*/
//===================================
/*
use std::ops::Neg;

fn main() {
    let num = 2;

    if num >= 2 {
        finn_plus(num);
    } else if num < 0 {
        finn_neg(num);        
    } else {
        println!("y {} num 1", num);
    }
}

fn finn_plus(x:i32) {
    //let mut num = 1;
    let mut num_f = 0;
    let mut num_0 = 1;
    let mut num_1 = 0;

    for y in 1..(x+1) {
        
        let num = num_0 + num_1;
        num_0 = num;
        num_1 = num_f;
        num_f = num;
    println!("y {}, num {}", y, num);
    }    
}

fn finn_neg(x:i32) {
    //let mut num = 1;
    let mut num_f = 0;
    let mut num_0 = 1;
    let mut num_1 = 0;

    for y in 1..(x-1).neg() {
        
        let num = num_1 - num_0;
        num_0 = num;
        num_1 = num_f;
        num_f = num;
    println!("y {}, num {}", y.neg(), num.neg());
    }
}
*/

fn main () {
    //let s = "hello".to_owned();
    //let w = "world!";

    //println!("{}", s);

    //let s2 = s + " " + w;
    //println!("{}", s2);

    let i = &mut [1, 2, 3, 5];
    let r = &[4, 5, 6, 5];

    let mut k = 0;
    let mut y = &r[k];

        for j in i.iter_mut() {                                                    
                *j += y;
                k += 1;                    
                if k == r.len() {
                    k -= 1;
                    y = &0;
                } else { y = &r[k]; }                                                  
        } 
        println!("{:?}", i);  
}
