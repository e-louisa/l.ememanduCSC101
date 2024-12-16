use std::io::Write;

fn main() {

    let stud1:[&str;4] = ["Oluchi Mordi","ACC10211111","Accounting","300"];
    let stud2:[&str;4] = ["Adams Aliyu","ECO10110101","Economics","100"];
    let stud3:[&str;4] = ["Shania Bolade","CSC10328828","Computer","200"];
    let stud4:[&str;4] = ["Adekunle Gold","EEE11020202","Electrical","200"];
    let stud5:[&str;4] = ["Blanca Edemoh","MEE10202001","Mechanical","100"];

    let mut file = std::fs::File::create("project2.txt").expect("create failed");
    file.write_all("Welcome to PAU SIMS.".as_bytes()).expect("write failed");
    file.write_all("\n The student information will be arranged in the following order:
        \n1.Student Name 2.Matric Number 3.Department 4.Level".as_bytes()).expect("write failed");

    let stud_1 = format!("\nStudent Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}",
        stud1[0], stud1[1], stud1[2], stud1[3]);
    file.write_all(stud_1.as_bytes()).expect("write failed");

    let stud_2 = format!("\nStudent Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}",
        stud2[0], stud2[1], stud2[2], stud2[3]);
    file.write_all(stud_2.as_bytes()).expect("write failed");

    let stud_3 = format!("\nStudent Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}",
        stud3[0], stud3[1], stud3[2], stud3[3]);
    file.write_all(stud_3.as_bytes()).expect("write failed");

    let stud_4 = format!("\nStudent Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}",
        stud4[0], stud4[1], stud4[2], stud4[3]);
    file.write_all(stud_4.as_bytes()).expect("write failed");

    let stud_5 = format!("\nStudent Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}",
        stud5[0], stud5[1], stud5[2], stud5[3]);
    file.write_all(stud_5.as_bytes()).expect("write failed");

    println!("file created succesfully.");

}
