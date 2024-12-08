use std::io;

fn main() {
    println!("Welcome to Ernst & Young Global Limited.");
    println!("We are here to find the most experienced developers to work for us.");
    highexpdev();
}

fn name(mut candname: String) -> String {
    println!("What is your name? ");
    io::stdin().read_line(&mut candname).expect("Not a valid string.");
    return candname.trim().to_string();
}

fn exp(mut years: String) -> i32 {
    println!("How many years of work experience do you have? ");
    let mut years = String::new();
    io::stdin().read_line(&mut years).expect("Not a valid string");
    let years: i32 = years.trim().parse().expect("Not a valid number");
    return years;
}

fn candinfo() -> (String, i32) {
    let candidate = (
        name(String::new()),
        exp(String::new()),
    );
    println!("Your full name is: {}", candidate.0);
    println!("You have {} years of experience.", candidate.1);
    return candidate;
}

fn highexpdev() {
    let mut vector: Vec<(String, i32)> = Vec::new();
    let mut i: i32 = 0;

    while i < 5 {
        println!("\nYou are developer {}", i + 1);
        vector.push(candinfo());
        i += 1;
    }

    let mut highest: i32 = 0;

    for (_, month_exp) in &vector {
        if *month_exp > highest {
            highest = *month_exp;
        }
    }

    println!("\nMost Experienced Programmer: ");

    let highest_vec: Vec<&(String, i32)> = vector
        .iter()
        .filter(|(_, exp)| *exp == highest)
        .collect();
    println!("{:?}", highest_vec);
}