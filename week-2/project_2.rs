fn main () 
{
	//state variables

	let	toshiba = 2 * 450000.00;

	let mac = 1 * 1500000.00;

	let hp = 3 * 750000.00;

	let dell = 3 * 2850000.00;

	let acer = 1 * 250000.00;

	//calculate sum
	let sum = (toshiba + mac + hp + dell + acer);
	//print sum
	println!("The sum is {}", sum);

	//calculate average
	let avg = sum / 4;

	//print average
	println!("The average is {}", avg);
}