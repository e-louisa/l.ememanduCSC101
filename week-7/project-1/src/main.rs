use std::io;

fn trapezium() {
    let mut t1 = String::new();
    println!("Enter the height of the trapezium: ");
    io::stdin().read_line(&mut t1).expect("Failed to read input");
    let h1:f32 = t1.trim().parse().expect("Invalid input");

    let mut t2 = String::new();
    println!("Enter the first base of the trapezium: ");
    io::stdin().read_line(&mut t2).expect("Failed to read input");
    let b1:f32 = t2.trim().parse().expect("Invalid input");

    let mut t3 = String::new();
    println!("Enter the second base of the trapezium: ");
    io::stdin().read_line(&mut t3).expect("Failed to read input");
    let b2:f32 = t3.trim().parse().expect("Invalid input");

    let area_t = h1/2.0 * (b1 + b2);
    println!("The area of the trapezium is {} units.", area_t);
}

fn rhombus() {
    let mut r1 = String::new();
    println!("Enter the length of the first diagonal: ");
    io::stdin().read_line(&mut r1).expect("Failed to read input");
    let d1:f32 = r1.trim().parse().expect("Invalid input");

    let mut r2 = String::new();
    println!("Enter the length of the second diagonal: ");
    io::stdin().read_line(&mut r2).expect("Failed to read input");
    let d2:f32 = r2.trim().parse().expect("Invalid input");

    let area_r = 0.5 * d1 * d2;
    println!("The area of the rhombus is {} units.", area_r);
}

fn parallelogram() {
    let mut p1 = String::new();
    println!("Enter the base of the parallelogram: ");
    io::stdin().read_line(&mut p1).expect("Failed to read input");
    let b1:f32 = p1.trim().parse().expect("Invalid input");

    let mut p2 = String::new();
    println!("Enter the first base of the trapezium: ");
    io::stdin().read_line(&mut p2).expect("Failed to read input");
    let a1:f32 = p2.trim().parse().expect("Invalid input");

    let area_p = b1 * a1;
    println!("The area of the parallelogram is {} units.", area_p);
}

fn cube() {
    let mut c1 = String::new();
    println!("Enter the length of the side: ");
    io::stdin().read_line(&mut c1).expect("Failed to read input");
    let l1:f32 = c1.trim().parse().expect("Invalid input");

    let area_c = 6.0 *l1 * l1;
    println!("The area of the cube is {} units.", area_c);
}

fn cylinder() {
    let mut cy1 = String::new();
    println!("Enter the radius of the cylinder: ");
    io::stdin().read_line(&mut cy1).expect("Failed to read input");
    let rad:f32 = cy1.trim().parse().expect("Invalid input");

    let mut cy2 = String::new();
    println!("Enter the height of the cylinder: ");
    io::stdin().read_line(&mut cy2).expect("Failed to read input");
    let height:f32 = cy2.trim().parse().expect("Invalid input");

    let area_cy = 3.14 * rad * rad * height;
    println!("The area of the cylinder is {} units.", area_cy);
}

fn main() {
    println!("Welcome to MTH 101 Class! What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Not a valid string");
    println!("Welcome to MTH 101, {}", name);

    println!("What shape's area do you want to calculate today?");
    println!("Choose from trapezium, rhombus, parallelogram, cube, cylinder.");
    let mut shape = String::new();
    io::stdin().read_line(&mut shape).expect("Not a valid string");

    if shape.trim().to_lowercase() == "trapezium" {
        trapezium()
    }
    else if shape.to_lowercase().trim() == "rhombus" {
        rhombus()
    }
    else if shape.to_lowercase().trim() == "parallelogram" {
        parallelogram()
    }
    else if shape.to_lowercase().trim() == "cube" {
        cube()
    }
    else if shape.to_lowercase().trim() == "cylinder" {
        cylinder()
    }

}