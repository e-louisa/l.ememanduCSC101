use std::io::Read;

fn main(){
    let mut file = std::fs::File::open(r"C:\Users\louis\Desktop\staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}