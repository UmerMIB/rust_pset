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

    match str.as_ref(){
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


fn cal_interest(){
    println!("Please enter principal amount: ");
    let amount =  get_input().trim().parse::<i64>().unwrap();
    println!("Please Enter Rate of interest in %: ");
    let interest =  get_input().trim().parse::<f64>().unwrap();
    println!("Enter number of years for investment: ", );
    let years =  get_input().trim().parse::<i64>().unwrap();

    let rate = (amount as f64 * interest * years as f64)/100.0;
    
    println!("After {} years your principal amount {} over an interest rate of {} % will be {}", &years, &amount, &interest, &rate);
}

fn euclidean_distance(){

    println!("Enter Co-ordinate for x1: ");
    let x1 =  get_input().trim().parse::<i64>().unwrap();

    println!("Enter Co-ordinate for x2: ");
    let x2 =  get_input().trim().parse::<i64>().unwrap();

    println!("Enter Co-ordinate for y1: ");
    let y1 =  get_input().trim().parse::<i64>().unwrap();

    println!("Enter Co-ordinate for y2: ");
    let y2 =  get_input().trim().parse::<i64>().unwrap();

    let mut distance = ((&y1-&x1)*(&y1-&x1))+((&y2-&x2)*(&y2-&x2));

    println!("Distance between points ({}, {}) and ({}, {}) is {}",y1,x1,y2,x2, (distance as f64).sqrt());
}


fn feet_cm(){

    println!("Enter Height in Feet: ");
    let feet =  get_input().trim().parse::<i64>().unwrap();

    let cm:f64 = (feet as f64)*30.48;

    println!("There are {} Cm in {} ft ",cm,feet);
}

fn bmi(){

        println!("Enter Height in CM: ");
    let height =  get_input().trim().parse::<i64>().unwrap();
        println!("Enter Weight in KG: ");
    let weight =  get_input().trim().parse::<i64>().unwrap();

    let bmi:f64 = (weight/(height*height)) as f64;

    println!("Your BMI is {}",bmi);
}


fn sum_of_positive_integers(){

    println!("Enter value of n: ");
    let mut number =  get_input().trim().parse::<i64>().unwrap();
    let mut sum = 0;
    while number != 0 {
        sum = sum + number;
        number = number - 1;

    }
    println!("Sum of n Positive integers till {} is {} ",number, sum);
}

fn sum_of_number(){

    println!("Enter value of n: ");
    let mut number =  get_input().trim().parse::<i64>().unwrap();
    let mut sum = 0;
    let mut temp;

    print!("Sum of ");
	    while number!=0
	    {
	        temp=number%10;
	        sum=sum+temp;
            number = number / 10;
            print!("{} + ",&temp);

	    }
    print!("is {}",sum);

}

fn decimal_to_binary(){

    println!("Enter a Decimal number: ");
    let mut decimal =  get_input().trim().parse::<i64>().unwrap();

    println!("Binary Representation of {} is {:b}", decimal, decimal);
}

fn binary_to_decimal(){

    println!("Enter a Binary number: ");
    let mut num =  get_input().trim().parse::<i64>().unwrap();
    let (mut decimal, mut base) = (0,1);
    let mut rem;

    let binary = num;
    while num > 0 {
        rem = num % 10;
        decimal = decimal + rem * base;
        num = num / 10 ;
        base = base * 2;
    }
    println!("Decimal Representation of {} is {}", binary, decimal);
}

fn vowels_consonants() {

    println!("Enter Text: ");
    let string =  get_input();

    let (mut vowels, mut consonants) = (0,0);

    for str in string.chars() {
        match str {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => vowels += 1,
            _ => consonants += 1,
        }
    }

    println!("\nNumber = {}", vowels);
    println!("\nAlphabets = {}", consonants);

}

fn is_palindrom(){
    println!("Enter a Binary number: ");
    let string =  get_input();

    let s = string.to_string().to_lowercase().chars().filter(|&c| c.is_alphanumeric()).collect::<String>();

    if s == s.chars().rev().collect::<String>() {
        print!("Text {} is Palindrome", &string);
    } else {
        print!("Text {} is not Palindrome", &string);
    }
}


fn count_data_types() {

    println!("Enter Text: ");
    let string =  get_input();

    let (mut number, mut aplhabet, mut space, mut special_character) = (0,0,0,0);

    for str in string.chars() {
        match str {
            'a'...'z' | 'A'...'Z' => aplhabet += 1,
            '0'...'9' => number += 1,
            ' ' => space += 1,
            _ => special_character += 1,
        }
    }

    println!("\nNumber = {}", number);
    println!("\nAlphabets = {}", aplhabet);
    println!("\nSpecial Characters = {}", special_character);
    println!("\nSpaces = {}", space);
}

fn pattern20(){
    for row in 1..7
    {
        for _b in 1..row
        {
            print!("*");
        }
        println!("");   
    }
    for row in (1..6).rev()
    {
        for _b in 1..row
        {
            print!("*");
        }
        println!("");   
    }
}

fn pattern21(){
        for row in 1..7
    {
        for _b in 1..row
        {
            print!("{}",_b);
        }
        println!("");   
    }
    for row in (1..6).rev()
    {
        for _b in 1..row
        {
            print!("{}",_b);
        }
        println!("");   
    }

}

fn pattern22(){
     for row in 0..10
    {
        for _b in 0..row
        {
            print!("{}",row);
        }
        println!("");   
    }
}


fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to Input");
    buffer
}