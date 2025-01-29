use std::io;
use std::io::Read;

fn administrator(){
    let mut file = std::fs::File::open(r"C:\Users\louis\Desktop\globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn project_manager(){
    let mut file = std::fs::File::open(r"C:\Users\louis\Desktop\project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn employee(){
    let mut file = std::fs::File::open(r"C:\Users\louis\Desktop\staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn customer(){
    let mut file = std::fs::File::open(r"C:\Users\louis\Desktop\customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn vendor(){
    let mut file = std::fs::File::open(r"C:\Users\louis\Desktop\dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn main(){

    let users = vec!["Administrator","Project Manager","Employee","Customer","Vendor"];
    println!("Welcome to the Globacom Ltd Database.");
    println!("What category do you belong to?");
    for (index, users) in users.iter().enumerate() {
        println!("{}. {}", index + 1, users);
    }

    println!("\n\nSelect the corresponding number (e.g 1,2...)");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Invalid input");
    let num:u64 = num.trim().parse().expect("Failed to read input");

    println!("The corresponding data for your category is:- ");

    if num == 1 {
        administrator();
    }
    else if num == 2 {
        project_manager();
    }
    else if num == 3 {
        employee();
    }
    else if num == 4 {
        customer();
    }
    else if num == 5 {
        vendor();
    }
}