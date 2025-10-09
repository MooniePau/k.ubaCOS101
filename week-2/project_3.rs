fn main ()
{
	//state variables
	let p:f64 = 510000.00;
	let r:f64 = 5.00;
	let t:f64 = 3.00;

	//find the derpreciation

	let a = p * (1.0 - (r / 100.0)).powf(t);

	let dep = p - a;

	//print depreciation

	println!("The depreciation is {}", dep);
}