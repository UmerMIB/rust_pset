fn main() {
    println!("Hello, world!");
    println!("Area of Circle {}",area(2));

    is_negative(0)
}

fn area(radius: i64) -> f64 {
        std::f64::consts::PI * (radius as f64 * radius as f64)
}

fn is_negative(num: i64) {

     if num > 0 { println!("Positive Number"); } else if num < 0 { println!("Negative Number"); } else {println!("Zero Entered")};

}
