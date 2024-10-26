fn main() {
	let p = 510000;
	let r = 5;
	let n = 3;

	//the value
	let v = p * ( 1 - (r / 100)) ^ n;
	println!("The value of the tv is {}", v);
	
}