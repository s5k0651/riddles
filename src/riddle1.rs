// Riddle: If 1/2 of 5 is 3, then what is 1/3 of 10?
// Write a rust code that is generic for the inputs and that ALWAYS solves the riddle.

use std::io;
use std::str::FromStr;

pub fn read<T: FromStr>(t: &mut T) {
    let mut buf = String::new();
    if let Ok(_) = io::stdin().read_line(&mut buf) {
        if let Ok(val) = buf.trim().trim_end().parse::<T>() {
            *t = val;
        }
    }
}

// According to the riddle... 1/2 =? 60% ==> 1 =? 120%
// so let modifier be 1.2 ~ 120%
// Otherwise simple solution can be done using ceil method (rounding).

fn func1(numerator: u8, denominator: u8, n: u8) -> u8 {
    let res = 1.2 * (numerator as f32 / denominator as f32) * n as f32;
    println!("Answer is: {}", res as u8);
    res as u8
}

fn func2(dec_val: f32, n: f32) -> u8 {
    let res = 1.2 * dec_val * n;
    println!("Answer is: {}", res as u8);
    res as u8
}

fn func3(fraction: String, n: String) -> u8 {
    let mut data: Vec<usize> = vec![];
    for part in fraction.split('/') {
        data.push(part.parse().unwrap());
    }
    data.push(n.parse().unwrap());
    if data.len() != 3 {
        panic!(" Expected a string like (eg. 1/3)");
    }
    let res = 1.2 * (data[0] as f32 / data[1] as f32) * data[2] as f32;
    println!("Answer is: {}", res as u8);
    res as u8
}

fn input_type1(mut numerator: u8, mut denominator: u8, mut n: u8) {
    println!("Enter the Numerator ");
    read(&mut numerator);
    println!("Enter the Denominator ");
    read(&mut denominator);
    println!("{}/{} of Number", numerator, denominator);
    println!("Enter the Number");
    read(&mut n);
    println!("Fraction: {}/{}, Number: {}", numerator, denominator, n);
    func1(numerator, denominator, n);
}

fn input_type2(mut dec_val: f32, mut n: f32) {
    println!("Enter in Decimal form: ");
    read(&mut dec_val);
    println!("Enter the Number");
    read(&mut n);
    println!("Decimal Value: {}, Number: {}", dec_val, n);
    func2(dec_val, n);
}

fn input_type3(mut fraction: String, mut n: String) {
    println!("Enter the Fraction (eg. 1/3) ");
    read(&mut fraction);
    println!("Enter the Number");
    read(&mut n);
    println!("Fraction: {}, Number: {}", fraction, n);
    func3(fraction, n);
}

pub fn solve_riddle_1() {
    println!("-----------------------------------------------------------");
    println!("---| Riddle: If 1/2 of 5 is 3, then what is 1/3 of 10? |---");
    println!("-----------------------------------------------------------");
    println!("Select an option: ");
    println!("-----------------------------------------------------------");
    println!("1) Enter the Fraction (Numerator, Denominator) & the Number");
    println!("2) Enter the Fraction in Decimal Form & the Number");
    println!("3) Enter the Fraction in String Format (eg. 1/3) & the Number");
    println!("-------------------------------------------------------------");

    let numerator: u8 = 1;
    let denominator: u8 = 1;
    let n: u8 = 1;
    let dec_val: f32 = 0.0;
    let n1: f32 = 0.0;
    let fraction = String::new();
    let n2 = String::new();
    let mut choice = 0;

    read(&mut choice);

    match choice {
        1 => input_type1(numerator, denominator, n),
        2 => input_type2(dec_val, n1),
        3 => input_type3(fraction, n2),
        _ => println!("No Types Found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_func1() {
        assert_eq!(func1(1, 2, 5), 3);
    }

    #[test]
    fn test2_func1() {
        assert_eq!(func1(1, 3, 10), 4);
    }

    #[test]
    fn test3_func2() {
        assert_eq!(func2(0.33333333, 10.0), 4);
    }

    #[test]
    fn test4_func2() {
        assert_eq!(func2(0.5, 5.0), 3);
    }

    #[test]
    fn test5_func3() {
        assert_eq!(func3("1/2".to_string(), "5".to_string()), 3);
    }

    #[test]
    fn test6_func3() {
        assert_eq!(func3("1/3".to_string(), "10".to_string()), 4);
    }
}
