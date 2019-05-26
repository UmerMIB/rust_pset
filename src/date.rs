// extern crate chrono;

// use std::io;
// use chrono::{ NaiveDate};
// use chrono::format::ParseError;

// fn main() -> Result<(), ParseError> {

//     let mut dt = String::new();

//     println!("Enter a date in (format 2015-09-05 YYY-MM-DD) ");
//     io::stdin().read_line(&mut dt).expect("\nProblem reading data");

//     let date1 = NaiveDate::parse_from_str(&dt.trim(), "%Y-%m-%d")?;

//     let mut dt2 = String::new();

//     println!("Enter a date in (format 2015-09-05 YYY-MM-DD) ");
//     io::stdin().read_line(&mut dt2).expect("\nProblem reading data");

//     let date2 = NaiveDate::parse_from_str(&dt2.trim(), "%Y-%m-%d")?;
    
//     let dif = date2.signed_duration_since(date1);

//     println!("There are {:?} days in between {} and {}",dif.num_days(),date1,date2);

//     Ok(())
// }