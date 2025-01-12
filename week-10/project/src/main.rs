struct Laptops{
    hp_cost:u32, ibm_cost:u32, toshiba_cost:u32, dell_cost:u32
}

impl Laptops {
    fn total_cost(&self)-> u32 {
        3 * self.hp_cost + 3 * self.ibm_cost + 3 * self.toshiba_cost + 3 * self.dell_cost
    }
}

fn main() {
    let cost = Laptops {
        hp_cost:650000,
        ibm_cost:755000,
        toshiba_cost:550000,
        dell_cost:850000
    };

    //printing the final result
    println!("The total cost of the HP laptops is {}", 3 * cost.hp_cost);
    println!("The total cost of the IBM laptops is {}", 3 * cost.ibm_cost);
    println!("The total cost of the Toshiba laptops is {}", 3 * cost.toshiba_cost);
    println!("The total cost of the Dell laptops is {}", 3 * cost.dell_cost);
    println!("The total cost of the man's purchase is {}", cost.total_cost());
}