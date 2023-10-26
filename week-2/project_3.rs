fn main() {
	let p:f32 = 210_000.00;
	let r:f32 = 0.05;
	let n:f32 = 3.00;


	// Compound interest
	let a = p * ( 1.0 - r).powf(n);
	println!("The Amount is {}",a);
	let d = p - a;
	println!("The value of the the TV is {}",d);
}