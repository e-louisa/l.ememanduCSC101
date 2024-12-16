use std::io::Write;

fn main() {

    let lager = "33 Export, Desperados, Goldberg, Gulder, Heineken, Star";
    let stout = "Legend, Turbo King, Williams";
    let non_alcholic = "Maltina, Amstel Malta, Malta Gold, Fayrouz";

    //creating the file
    let mut file = std::fs::File::create("project1.txt").expect("create failed");
    file.write_all("Welcome to Nigerian Brewery Limited.\nWe have an excellent assortment of drinks, including:"
        .as_bytes()).expect("write failed");
    file.write_all("\nFor Lager, we have: ".as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all("\nFor Stout, we have: ".as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all("\nFor Non alcoholic, we have: ".as_bytes()).expect("write failed");
    file.write_all(non_alcholic.as_bytes()).expect("write failed");

    println!("Data written to file.");
}
