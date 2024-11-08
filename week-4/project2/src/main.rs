use std::io;

fn main() {

    println!("Are you experienced or inexperienced? ");
    let mut exp = String::new();
    io::stdin().read_line(&mut exp).expect("Not a valid string");

    println!("How old are you? ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age: f32 = input2.trim().parse().expect("Not a valid number");

    if exp.trim() == "experienced" && age >= 40.0 {
        println!("Your employee incentive is N1,560,000.");
    } else if exp.trim() == "experienced" && age >= 30.0 && age < 40.0 {
        println!("Your employee incentive is N1,480,000.");
    } else if exp.trim() == "inexperienced" {
        println!("Your employee incentive is N100,000.");
    } else {
        println!("Your employee incentive is N1,300,000.");
    }
}
