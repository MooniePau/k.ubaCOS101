fn main() {

	//state variables

	let p:f64 = 520000000.0;
	let n:f64 = 5.0;
	let r:f64 = 10.0; 


	//calculate compund interest

	let a = p * (1.0 + (r / 100.0)) * n;
	let ci = a - p;


	//print compound interest

	println!("Compound Interest is {}", ci);
	
}