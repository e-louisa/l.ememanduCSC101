use std::io::Write;

fn main() {

    //creating the arrays
    let headings = ["Name of Commissioner","Ministry","Geopolitical Zone"];
    let name_of_commissioner = ["Aigbogun Alama Daudu","Murtala Afeez Bendu",
    "Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = ["internal affairs","justice","defense","power & steel","petroleum"];
    let geopolitical_zone = ["South West","North East","South South","South West","South East"];

    //creating the text file
    let mut file = std::fs::File::create("project3.txt").expect("create failed");

    file.write_all("This is the result of all three datasets being merged into one single output.
        \nThe data will be written in the following order:".as_bytes()).expect
    ("writing failed");

    let headings_ = format!("\n{}, {}, {}\n", headings[0], headings[1], headings[2]);
    file.write_all(headings_.as_bytes()).expect("writing failed");

    let info_0 = format!("\n{}, {}, {}", name_of_commissioner[0], ministry[0], geopolitical_zone[0]);
    file.write_all(info_0.as_bytes()).expect("write failed");

    let info_1 = format!("\n{}, {}, {}", name_of_commissioner[1], ministry[1], geopolitical_zone[1]);
    file.write_all(info_1.as_bytes()).expect("write failed");

    let info_2 = format!("\n{}, {}, {}", name_of_commissioner[2], ministry[2], geopolitical_zone[2]);
    file.write_all(info_2.as_bytes()).expect("write failed");

    let info_3 = format!("\n{}, {}, {}", name_of_commissioner[3], ministry[3], geopolitical_zone[3]);
    file.write_all(info_3.as_bytes()).expect("write failed");

    let info_4 = format!("\n{}, {}, {}", name_of_commissioner[4], ministry[4], geopolitical_zone[4]);
    file.write_all(info_4.as_bytes()).expect("write failed");

    println!("Datasets merged successfully.");

}
