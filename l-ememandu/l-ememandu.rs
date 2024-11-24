use std::io;

fn main() {

	println!("Enter your name: ");
	let mut name = String::new();
	io::stdin().read_line(& mut name).expect("Not a valid string");

	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut email = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut diagnosis = String::new();
	let mut village = String::new();

	println!("Enter your date and month of birth [DDMM]: ");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
    let dob:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your year of birth [YYYY]: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let year:f32 = input2.trim().parse().expect("Not a valid number");
    let age:f32 = 2024.0 - year;

    println!("Enter your email address: ");
	io::stdin().read_line(& mut email).expect("Not a valid string");

	println!("Enter your phone number: ");
	io::stdin().read_line(&mut input3).expect("Not a valid string");
    let phone:f32 = input3.trim().parse().expect("Not a valid number");

    println!("Enter your number of siblings: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let siblings:f32 = input4.trim().parse().expect("Not a valid number");

    println!("Enter your number of children: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let children:f32 = input5.trim().parse().expect("Not a valid number");

    println!("Enter your medical diagnosis [small letters only]: ");
    io::stdin().read_line(& mut diagnosis).expect("Not a valid string");

    println!("Enter your village of residence [small letters only]: ");
    io::stdin().read_line(& mut village).expect("Not a valid string");

    println!("What was your number among the patients who visited the clinic today?");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let position:f32 = input6.trim().parse().expect("Not a valid number");

    let sibchild:f32 = siblings + children;

    if diagnosis.trim() == "alzheimer" && age > 50.0 && children > 4.0 && village.trim() == "akpabom" && position <= 100.0 {
    	let discount1 = 0.8 * 1_200_000.0;
    	println!("The patient's name is {}", name);
    	println!("The patient was born on {}{}", dob, year);
    	println!("The patient is {} years old", age);
    	println!("The patient's email address is {}", email);
    	println!("The patient's phone number is {}", phone);
    	println!("The patient has {} siblings", siblings);
    	println!("The patient has {} children", children);
    	println!("The patient's diagnosis is {}", diagnosis);
    	println!("The patient's vilage of residence is {}", village);
    	println!("The patient has a 20% discount and therefore will pay the amount of: {}", discount1);
    }
    else if diagnosis.trim() == "alzheimer" {
    	println!("The patient's name is {}", name);
    	println!("The patient was born on {}{}", dob, year);
    	println!("The patient is {} years old", age);
    	println!("The patient's email address is {}", email);
    	println!("The patient's phone number is {}", phone);
    	println!("The patient has {} siblings", siblings);
    	println!("The patient has {} children", children);
    	println!("The patient's diagnosis is {}", diagnosis);
    	println!("The patient's vilage of residence is {}", village);
    	println!("The patient will pay the amount of 1,200,000.")
    }
    else if diagnosis.trim() == "arrythmia" && age == 30.0 && siblings > 4.0 && village.trim() == "ngbauji" && position <= 100.0 {
    	let discount2 = 0.95 * 550_000.0;
    	println!("The patient's name is {}", name);
    	println!("The patient was born on {}{}", dob, year);
    	println!("The patient is {} years old", age);
    	println!("The patient's email address is {}", email);
    	println!("The patient's phone number is {}", phone);
    	println!("The patient has {} siblings", siblings);
    	println!("The patient has {} children", children);
    	println!("The patient's diagnosis is {}", diagnosis);
    	println!("The patient's vilage of residence is {}", village);
    	println!("The patient has a 5% discount and therefore will pay the amount of: {}", discount2);
    }
    else if diagnosis.trim() == "arrythmia" {
    	println!("The patient's name is {}", name);
    	println!("The patient's diagnosis is {}", diagnosis);
    	println!("The patient's village of residence is {}", village);
    	println!("The patient's email address is {}", email);
    	println!("The patient was born on {}{}", dob, year);
    	println!("The patient is {} years old", age);
    	println!("The patient's phone number is {}", phone);
    	println!("The patient has {} siblings", siblings);
    	println!("The patient has {} children", children);
    	println!("The patient will pay the amount of 550,000.")
	}
	else if diagnosis.trim() == "chronic kidney disease" && age > 40.0 && sibchild > 3.0 && village.trim() == "atabrikang" && position <= 100.0 {
    	let discount3 = 0.85 * 1_500_000.0;
    	println!("The patient's name is {}", name);
    	println!("The patient was born on {}{}", dob, year);
    	println!("The patient is {} years old", age);
    	println!("The patient's email address is {}", email);
    	println!("The patient's phone number is {}", phone);
    	println!("The patient has {} siblings", siblings);
    	println!("The patient has {} children", children);
    	println!("The patient's diagnosis is {}", diagnosis);
    	println!("The patient's vilage of residence is {}", village);
    	println!("The patient has a 15% discount and therefore will pay the amount of: {}", discount3);
    }
    else if diagnosis.trim() == "chronic kidney disease" {
    	println!("The patient's name is {}", name);
    	println!("The patient was born on {}{}", dob, year);
    	println!("The patient is {} years old", age);
    	println!("The patient's email address is {}", email);
    	println!("The patient's phone number is {}", phone);
    	println!("The patient has {} siblings", siblings);
    	println!("The patient has {} children", children);
    	println!("The patient's diagnosis is {}", diagnosis);
    	println!("The patient's vilage of residence is {}", village);
    	println!("The patient will pay the amount of 1,500,000.");
    }
    else if diagnosis.trim() == "diabetes" && age > 28.0 && age < 45.0 && children >= 2.0 && children <= 4.0 && village.trim() == "okorobilom" && position <= 100.0 {
    	let discount4 = 0.9 * 800_000.0;
    	println!("The patient's name is {}", name);
    	println!("The patient was born on {}{}", dob, year);
    	println!("The patient is {} years old", age);
    	println!("The patient's email address is {}", email);
    	println!("The patient's phone number is {}", phone);
    	println!("The patient has {} siblings", siblings);
    	println!("The patient has {} children", children);
    	println!("The patient's diagnosis is {}", diagnosis);
    	println!("The patient's vilage of residence is {}", village);
    	println!("The patient has a 10% discount and therefore will pay the amount of: {}", discount4);
    }
    else if diagnosis.trim() == "diabetes" {
    	println!("The patient's name is {}", name);
    	println!("The patient was born on {}{}", dob, year);
    	println!("The patient is {} years old", age);
    	println!("The patient's email address is {}", email);
    	println!("The patient's phone number is {}", phone);
    	println!("The patient has {} siblings", siblings);
    	println!("The patient has {} children", children);
    	println!("The patient's diagnosis is {}", diagnosis);
    	println!("The patient's vilage of residence is {}", village);
    	println!("The patient will pay the amount of 800,000.");
    }
    else if diagnosis.trim() == "arthritis" && age > 58.0 && sibchild > 5.0 && village.trim() == "emeremen" && position <= 100.0 {
    	let discount5 = 0.9 * 450_000.0;
    	println!("The patient's name is {}", name);
    	println!("The patient was born on {}{}", dob, year);
    	println!("The patient is {} years old", age);
    	println!("The patient's email address is {}", email);
    	println!("The patient's phone number is {}", phone);
    	println!("The patient has {} siblings", siblings);
    	println!("The patient has {} children", children);
    	println!("The patient's diagnosis is {}", diagnosis);
    	println!("The patient's vilage of residence is {}", village);
    	println!("The patient has a 10% discount and therefore will pay the amount of: {}", discount5);
    }
    else if diagnosis.trim() == "arthritis" {
    	println!("The patient's name is {}", name);
    	println!("The patient was born on {}{}", dob, year);
    	println!("The patient is {} years old", age);
    	println!("The patient's email address is {}", email);
    	println!("The patient's phone number is {}", phone);
    	println!("The patient has {} siblings", siblings);
    	println!("The patient has {} children", children);
    	println!("The patient's diagnosis is {}", diagnosis);
    	println!("The patient's vilage of residence is {}", village);
    	println!("The patient will pay the amount of 450,000.");
    }

}