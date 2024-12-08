use std::io;

fn main() {
    let division = vec!["Office Administrator","Academic","Lawyer","Teacher"];
    let positions = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    let offadmn = vec!["Intern","Administrator","Senior Administrator", "Office manager", "Director", "CEO"];
    let academic = vec!["Research assistant", "PhD candidate", "Post-doc researcher", "Senior lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior associate", "associate", "Senior associate 1-2", "Senior associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom teacher", "Senior teacher", "Leading teacher", "Deputy principal", "principal"];

// Asking the user for how much work experience they have.
    println!("Enter the number of years of experience you have: ");
    let mut years = String::new();
    io::stdin().read_line(&mut years).expect("Not a valid string");
    let years:f32 = years.trim().parse().expect("Not a valid number");

//asking what division the person is in
    println!("What division are you in? ");
    println!("The divisions are: {:?}", division);
    let mut job = String::new();
    io::stdin().read_line(&mut job).expect("Not a valid string");

//asking the person what subdivision they are in

    if job.trim().to_lowercase() == "office administrator" {
        println!("What subdivision are you in? ");
        println!("The subdivisions are: \n{:?}", offadmn);
        let mut offadmntype = String::new();
        io::stdin().read_line(&mut offadmntype).expect("Not a valid string");
        let offadmntype = offadmntype.trim().to_lowercase();
        println!("You are a {} office administrator.", offadmntype);

        //matching the type of office administrator to the position
        if offadmntype.trim().to_lowercase() == "intern" && years >= 1.0 && years <= 2.0 {
            println!("Your position is {:?}", positions[0]);
        }
        else if offadmntype.trim().to_lowercase() == "administrator" && years >= 3.0 && years <= 5.0 {
            println!("Your position is {:?}", positions[1]);
        }
        else if offadmntype.trim().to_lowercase() == "senior administrator" && years >= 5.0 && years <= 8.0 {
            println!("Your position is {:?}", positions[2]);
        }
        else if offadmntype.trim().to_lowercase() == "office manager" && years >= 8.0 && years <= 10.0 {
            println!("Your position is {:?}", positions[3]);
        }
        else if offadmntype.trim().to_lowercase() == "director" && years >= 10.0 && years <= 13.0 {
            println!("Your position is {:?}", positions[4]);
        }
        else if offadmntype.trim().to_lowercase() == "ceo" {
            println!("Your position is {:?}", positions[5]);
        }
        else {
            println!("Sorry, there's no position available for you ;( ");
        }
    }

    else if job.trim().to_lowercase() == "academic" {
        println!("What subdivision are you in? ");
        println!("The subdivisions are: \n{:?}", academic);
        let mut academictype = String::new();
        io::stdin().read_line(&mut academictype).expect("Not a valid string");
        let academictype = academictype.trim().to_lowercase();
        println!("You are a {} academic.", academictype);

        if academictype.trim().to_lowercase() == "research assistant" && years >= 3.0 && years <= 3.0 {
            println!("Your position is {:?}", positions[1]);
        }
        else if academictype.trim().to_lowercase() == "phd candidate" && years >= 5.0 && years <= 8.0 {
            println!("Your position is {:?}", positions[2]);
        }
        else if academictype.trim().to_lowercase() == "post doc researcher" && years >= 8.0 && years <= 10.0 {
            println!("Your position is {:?}", positions[3]);
        }
        else if academictype.trim().to_lowercase() == "senior lecturer" && years >= 10.0 && years <= 13.0 {
            println!("Your position is {:?}", positions[4]);
        }
        else if academictype.trim().to_lowercase() == "dean" {
            println!("Your position is {:?}", positions[5]);
        }
        else {
            println!("Sorry there's no position available for you ;( ");
        }

    }

    else if job.trim().to_lowercase() == "lawyer" {
        println!("What subdivision are you in? ");
        println!("The subdivisions are: \n{:?}", lawyer);
        let mut lawyertype = String::new();
        io::stdin().read_line(&mut lawyertype).expect("Not a valid string");
        let lawyertype = lawyertype.trim().to_lowercase();
        println!("You are a {} lawyer.", lawyertype);

        if lawyertype.trim().to_lowercase() == "paralegal" && years >= 1.0 && years <= 2.0 {
            println!("Your position is {:?}", positions[0]);
        }
        else if lawyertype.trim().to_lowercase() == "junior associate" && years >= 3.0 && years <= 5.0 {
            println!("Your position is {:?}", positions[1]);
        }
        else if lawyertype.trim().to_lowercase() == "associate" && years >= 5.0 && years <= 8.0 {
            println!("Your position is {:?}", positions[2]);
        }
        else if lawyertype.trim().to_lowercase() == "senior associate 1-2" && years >= 8.0 && years <= 10.0 {
            println!("Your position is {:?}", positions[3]);
        }
        else if lawyertype.trim().to_lowercase() == "senior associate 3-4" && years >= 10.0 && years <= 13.0 {
            println!("Your position is {:?}", positions[4]);
        }
        else if lawyertype.trim().to_lowercase() == "partner" {
            println!("Your position is {:?}", positions[5]);
        }
        else {
            println!("Sorry there's no position available for you ;( ")
        }
    }

    else if job.trim().to_lowercase() == "teacher" {
        println!("What subdivision are you in? ");
        println!("The subdivisions are: \n{:?}", teacher);
        let mut teachertype = String::new();
        io::stdin().read_line(&mut teachertype).expect("Not a valid string");
        let teachertype = teachertype.trim().to_lowercase();
        println!("You are a {} teacher.", teachertype);

        if teachertype.trim().to_lowercase() == "placement" && years >= 1.0 && years <= 2.0 {
            println!("Your position is {:?}", positions[0]);
        }
        else if teachertype.trim().to_lowercase() == "classroom teacher" && years >= 3.0 && years <= 5.0 {
            println!("Your position is {:?}", positions[1]);
        }
        else if teachertype.trim().to_lowercase() == "snr teacher" && years >= 5.0 && years <= 8.0 {
            println!("Your position is {:?}", positions[2]);
        }
        else if teachertype.trim().to_lowercase() == "leading teacher" && years >= 8.0 && years <= 10.0 {
            println!("Your position is {:?}", positions[3]);
        }
        else if teachertype.trim().to_lowercase() == "deputy principal" && years >= 10.0 && years <= 13.0 {
            println!("Your position is {:?}", positions[4]);
        }
        else if teachertype.trim().to_lowercase() == "principal" {
            println!("Your position is {:?}", positions[5]);
        }
        else {
            println!("Sorry there's no position available for you ;( ")
        }
    }
    
    else {
        println!("Sorry, there's no position available for you ;( ");
    }


        }