use std::convert::AsRef;

fn main() {
    println!("Hello, world!");

    // area();
    // is_negative();

    // is_divisble();

    // sphere_volume();

    // copy_string();

    // is_vowel();
}

fn area() {

    println!("Please Enter Radius of Circle: ");

    let radius =  get_input().trim().parse::<i64>().unwrap();

            
    let area = std::f64::consts::PI * (radius as f64 * radius as f64);

    println!("Area of Circle {}",area);

}

fn is_negative() {

    println!("Please Enter a Number: ");

    let num =  get_input().trim().parse::<i64>().unwrap();

    if num > 0 { println!("Positive Number"); } else if num < 0 { println!("Negative Number"); } else {println!("Zero Entered")};

}

fn is_divisble() {

    println!("Enter Numerator: ");
    let numerator =  get_input().trim().parse::<i64>().unwrap();

    println!("Enter Denominator: ");
    let denominator =  get_input().trim().parse::<i64>().unwrap();

    if (numerator%denominator)==0 {
    println!("Number {} is Completely divisible by {} ", numerator, denominator);
    } else {
            println!("Number {} is not Completely divisible by {} ", numerator, denominator);
    }
}

fn sphere_volume() {

    println!("Enter Radius of Sphere: ");

    let radius =  get_input().trim().parse::<i64>().unwrap();

            
    let sphere = (4.0/3.0)  * std::f64::consts::PI * (radius as f64 * radius as f64 * radius as f64);

    println!("Volume of the Sphere with Radius {} is {}",radius, sphere);

}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to Input");
    buffer
}

fn copy_string(){

    println!("Enter String: ");
    let str =  get_input();

    println!("How many copies of String you need: ");
    let count =  get_input().trim().parse::<usize>().unwrap();
    
    let mut copies = str.repeat(count);

    println!("{} copies of {} are ", count, &str );
    print!("{}", &copies );

}


fn even_odd() {

    println!("Enter Number: ");
    let num =  get_input().trim().parse::<i64>().unwrap();

    if (num%2)==0 {
    println!("{} is Even ", &num);
    } else {
    println!("{} is Odd ", &num);
    }
}

fn is_vowel(){
    print!("Enter a Character: ");
    let mut str =  get_input();

match str.as_ref() {
        // Match several values
        "a" | "e" | "i" | "o" | "u" | "A" | "E" | "I" | "O" | "U" => println!("Letter {}  is Vowel: ", &str),
        _ => println!("{} is Not a Vowel ", &str),

    }
}


fn triangle_area(){
    println!("Enter magnitude of Triangle Base: ");
    let base =  get_input().trim().parse::<i64>().unwrap();

    println!("Enter Magnitude of Triangle Height: ");
    let height =  get_input().trim().parse::<i64>().unwrap();

    let area = (base * height)/2;
    
    println!("Area of a Triangle with Height {} and Base {} is {}", height, base, area);
}


