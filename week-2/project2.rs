fn main () {
	let t = 450000;
	let m = 1500000;
	let h = 750000;
	let d = 2850000;
	let a = 250000;

	//to calculate the sum
	let s = (2 * t) + m + (3 * h) + (3 * d) + a;
	println!("The sum is {}", s);
	//to calculate the average
	let avg = s / (2 + 1 + 3 + 3 + 1);
	println!("the average is {}", avg);

}