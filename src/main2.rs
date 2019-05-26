#[macro_use]
extern crate text_io;

extern crate rand;

use std::io;
use rand::Rng;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::cmp::Ordering;


fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}


fn main() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));
}

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_string);
}

fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("{:?}",vec);
}

fn main() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
     println!("{:?}",vec);
}

fn main() {
    let mut index = 0;
    let mut index_two = 0;
    let mut i_300=0;
    let mut i_600=0;
    let mut i_above=0;
    let mut vec: Vec<f64> = Vec::new();
    while index <= 10 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0, 900.0);
        vec.push(x);
        print!("{}",x);
        index = index + 1;
    }
    while index_two <= 10 {
        if vec[index_two] <=300.0{
            i_300 += 1;
        }
        else if vec[index_two] <=600.0{
            i_600 += 1;
        }
        else {
            i_above +=1;
        }
        index_two = index_two + 1;
    }

    println!("Integer: {:?}", vec);
    println!("0 to 300: {}, 300 to 600: {}, 600 above: {}",i_300,i_600,i_above);
}

fn main() {

    let mut index = 0;
    let mut index_two = 0;
    let mut con_i = 0;
    let mut sum_vec = 0;
    let mut vec: Vec<i32> = Vec::new();

    while index <= 10 {

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, 100);

        vec.push(x);
        print!("{}",x);

        while con_i < vec.len(){
            sum_vec += vec[con_i];
            con_i += 1;
        }
        if sum_vec > 90{
            break;
        }
        index = index + 1;
    }
    vec.sort();
    println!("Integer: {:?}", vec);

}


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 10);

    loop {
        println!("I'm thinking of a number from 1 to 10.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[macro_use]
extern crate text_io;

fn main() {
    println!("HERE COMES THE DICE! ");
    println!("Roll # 01");
    let num_one: i32 = read!();
    println!("Roll # 02");
    let num_two: i32 = read!();
    println!("The Total is: {}", num_one+num_two);
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    let mut count_index = 0;

    while count_index <=7 {
        println!("I'm thinking of a number from 1 to 100. You have 7 guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        count_index +=1;
    }
}

fn main() {

    let mut sum_number = 0;
    while true{
        println!("I will add up the numbers you give me");
         println!("Number# ");
        let num_one: i32 = read!();
        if num_one == 0{
            break;
        }
        sum_number += num_one;
        println!("The Total so far is: {}", sum_number);
    }
    println!("The Total is: {}", sum_number);
}


