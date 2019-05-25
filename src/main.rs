fn main() {
    println!("Hello, world!");
    println!("Area of Circle {}",area(2));
}

fn area(radius: i64) -> f64 {
        std::f64::consts::PI * (radius as f64 * radius as f64)
}
