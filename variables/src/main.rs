use std::io;
use std::cmp::Ordering;

const WEEKS_IN_MONTH: u32 = 4;

fn main() {
    println!("Let's calculate your salary in 1 hour");
    println!("Your salary per month?");

    let mut salary = String::new();

    io::stdin()
        .read_line(&mut salary)
        .expect("Enter correct salary");

    let salary: u32 = salary.trim().parse().unwrap();

    println!("How many days a week do you work?");

    let mut work_days = String::new();

    io::stdin()
        .read_line(&mut work_days)
        .expect("Enter correct days count");

    let work_days: u32 = work_days.trim().parse().unwrap();

    println!("How many hours per day do you work?");

    let mut hours_per_day = String::new();

    io::stdin()
        .read_line(&mut hours_per_day)
        .expect("Enter correct hours count");

    let hours_per_day: u32 = hours_per_day.trim().parse().unwrap();

    let salary_per_hour = salary / (work_days * WEEKS_IN_MONTH) / hours_per_day;

    println!("Your salary in 1 hour is: {}", salary_per_hour);
    println!("Your salary per day is: {}", salary_per_hour * hours_per_day);
    println!("Your salary per week is: {}", salary_per_hour * hours_per_day * work_days);
    match salary_per_hour.cmp(&1000) {
        Ordering::Less => println!("Not so good :("),
        Ordering::Greater => println!("Good!"),
        _ => {}
    };
}
