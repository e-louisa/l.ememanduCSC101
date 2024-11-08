// program to find the roots and discriminant of a quadratic equation
// and to determine the nature of the roots of the equation

use std::io;

fn main ()

{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");
    
    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");   

    let d:f32 = (b * b) - (4.0 * a * c);
    println!("The discriminant of the equation is: {}", d);

    let root1:f32 = ((0.0 - b) + d) / (2.0 * a);
    let root2:f32 = ((0.0 - b) - d) / (2.0 * a);
    println!("The roots of the equation are: {},{}", root1, root2);

    if d == 0.0
    {
        println!("This equation has exactly one real root.");
    }
    else if d > 0.0
    {
        println!("This equation has two distinct roots.");
    }
    else
    {
        println!("This equation has no real roots.")
    }
}