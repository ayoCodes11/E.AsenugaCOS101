fn main()
{
	let s:f64 = 450000.00 * 2.0 + 1500000.00 * 1.0 + 750000.00 * 3.0 + 2850000.00 * 3.0 + 250000.00 * 1.0;
	let q:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;

	let a = s/q;

	println!("Sum of sales is {}", s);
	println!("Average is {}", a);
}