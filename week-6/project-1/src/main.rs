use std::io;

fn main() {
    println!("Welcome to our restaurant.");
    println!("Today's menu is: ");

    let p = "Poundo Yam/Edinkaiko Soup [Code - P]";
    let price_p:f32 = 3200.0;
    let f = "Fried Rice & Chicken [Code - F]";
    let price_f:f32 = 3000.0;
    let a = "Amala & Ewedu Soup [Code - A]";
    let price_a:f32 = 2500.0;
    let e = "Eba & Egusi Soup [Code - E]"; 
    let price_e:f32 = 2000.0;
    let w = "White Rice & Stew [Code - W]";
    let price_w:f32 = 2500.0;

    println!("1 portion of {} for N{}", p, price_p);
    println!("1 portion of {} for N{}", f, price_f);
    println!("1 portion of {} for N{}", a, price_a);
    println!("1 portion of {} for N{}", e, price_e);
    println!("1 portion of {} for N{}", w, price_w);

    let mut total: f32 = 0.0;

    loop {
    println!("Enter the code of the item you would like to order (or 'done' to finish ordering): ");
    let mut order1 = String::new();
    io::stdin().read_line(&mut order1).expect("Not a valid string");

    if order1.trim() == "done" {
        break;
    }
    else if order1.trim() == "P" {
        println!("Enter the quantity you want to order: ");
        let mut qty1 = String::new();
        io::stdin().read_line(&mut qty1).expect("Not a valid string");
        let qty:f32 = qty1.trim().parse().expect("Not a valid number");
        let totalcost = price_p * qty;
        total += totalcost;
        println!("Your order currently costs N{}", total);
    }
    else if order1.trim() == "F" {
        println!("Enter the quantity you want to order: ");
        let mut qty1 = String::new();
        io::stdin().read_line(&mut qty1).expect("Not a valid string");
        let qty:f32 = qty1.trim().parse().expect("Not a valid number");
        let totalcost = price_p * qty;
        total += totalcost;
        let totalcost = price_f * qty;
        total += totalcost;
        println!("Your order currently costs N{}", total);
    }
    else if order1.trim() == "A" {
        println!("Enter the quantity you want to order: ");
        let mut qty1 = String::new();
        io::stdin().read_line(&mut qty1).expect("Not a valid string");
        let qty:f32 = qty1.trim().parse().expect("Not a valid number");
        let totalcost = price_p * qty;
        total += totalcost;
        let totalcost = price_a * qty;
        total += totalcost;
        println!("Your order currently costs N{}", total);
    }
    else if order1.trim() == "E" {
        println!("Enter the quantity you want to order: ");
        let mut qty1 = String::new();
        io::stdin().read_line(&mut qty1).expect("Not a valid string");
        let qty:f32 = qty1.trim().parse().expect("Not a valid number");
        let totalcost = price_p * qty;
        total += totalcost;
        let totalcost = price_e * qty;
        total += totalcost;
        println!("Your order currently costs N{}", total);
    }
    else if order1.trim() == "W" {
        println!("Enter the quantity you want to order: ");
        let mut qty1 = String::new();
        io::stdin().read_line(&mut qty1).expect("Not a valid string");
        let qty:f32 = qty1.trim().parse().expect("Not a valid number");
        let totalcost = price_p * qty;
        total += totalcost;
        let totalcost = price_w * qty;
        total += totalcost;
        println!("Your order currently costs N{}", total);
    }
}

    if total > 10_000.0 {
        total = total as f32 * 0.95;
        println!("Your total is more than N10,000, therefore you are entitled to a 5% discount.")
    }

    println!("The total cost of your order is N{}", total);

}


