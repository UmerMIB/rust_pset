// #[macro_use]
// extern crate text_io;
// use std::str;
 
// fn main() {
//     let mut flag: char = 'y';

//     loop {
//         if flag == 'n' || flag == 'n' {
//             break;
//         } else if flag == 'y' || flag == 'Y' {
//             println!("Enter The Data To Compute");
//             let data: String = read!();
//             //   println!("{}", data);
//             let mut vec: Vec<char> = Vec::new();
//             let mut count = 0;
//             for x in data.chars() {
//                 //  println!("{}", x);
//                 let y = (x.to_string()).parse::<char>().unwrap();
//                 vec.push(y);
//             }
//             while count < vec.len() {
//                 if vec[count] == '+' {
//                     //  println!("{}", count);
//                     let mut count_two = 0;
//                     let mut count_three = count + 1;
//                     let mut first_value = String::from("");
//                     let mut second_value = String::from("");

//                     while count_two < count {
//                         first_value.push(vec[count_two]);
//                         count_two += 1;
//                     }
//                     while count_three < vec.len() {
//                         second_value.push(vec[count_three]);
//                         count_three += 1;
//                     }
//                     println!(
//                         "The Result Is: {}",
//                         first_value.parse::<i32>().unwrap() + second_value.parse::<i32>().unwrap()
//                     );
//                 }
//                 if vec[count] == '-' {
//                     // println!("{}", count);
//                     let mut count_two = 0;
//                     let mut count_three = count + 1;
//                     let mut first_value = String::from("");
//                     let mut second_value = String::from("");

//                     while count_two < count {
//                         first_value.push(vec[count_two]);
//                         count_two += 1;
//                     }
//                     while count_three < vec.len() {
//                         second_value.push(vec[count_three]);
//                         count_three += 1;
//                     }
//                     println!(
//                         "The Result Is: {}",
//                         first_value.parse::<i32>().unwrap() - second_value.parse::<i32>().unwrap()
//                     );
//                 }
//                 if vec[count] == '/' {
//                     // println!("{}", count);
//                     let mut count_two = 0;
//                     let mut count_three = count + 1;
//                     let mut first_value = String::from("");
//                     let mut second_value = String::from("");

//                     while count_two < count {
//                         first_value.push(vec[count_two]);
//                         count_two += 1;
//                     }
//                     while count_three < vec.len() {
//                         second_value.push(vec[count_three]);
//                         count_three += 1;
//                     }
//                     println!(
//                         "The Result Is: {}",
//                         first_value.parse::<i32>().unwrap() / second_value.parse::<i32>().unwrap()
//                     );
//                 }
//                 if vec[count] == '^' {
//                     // println!("{}", count);
//                     let mut count_two = 0;
//                     let mut count_three = count + 1;
//                     let mut first_value = String::from("");
//                     let mut second_value = String::from("");

//                     while count_two < count {
//                         first_value.push(vec[count_two]);
//                         count_two += 1;
//                     }
//                     while count_three < vec.len() {
//                         second_value.push(vec[count_three]);
//                         count_three += 1;
//                     }
//                     let mut count_pow = 0;
//                     let mut pow_sum = 1;
//                     let fCV = first_value.parse::<i32>().unwrap();
//                     let sCV = second_value.parse::<i32>().unwrap();
//                     while count_pow < sCV {
//                         pow_sum *= fCV;
//                         count_pow += 1;
//                     }
//                     println!("The Result Is: {}", pow_sum);
//                 }
//                 count += 1;
//             }
//         } else {
//             println!("Invalid Selection!");
//         }
//         println!("Do you want to continue program y/n ?");
//         flag = read!();
//     }
// }