fn main() {
	let p = 520000;
	let r = 10;
	let n = 5;

	//amount
	let a = p * ( 1 + (r / 100)) ^ n;
	println!("Amount is {}", a);
	//compound interest
	let i = a - p;
	println!("compound interest is {}", i);

}