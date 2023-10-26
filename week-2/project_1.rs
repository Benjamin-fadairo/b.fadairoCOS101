fn main() {
	let p:f32 = 520_000_000.00;
	let r:f32 = 0.1;
	let n:f32 = 5.0;


	// Simple interest
	let a = p * ( 1.0 + r).powf(n);
	println!("The Amount is {}",a);
	let ci = a - p;
	println!("The compound interest is {}",ci);
}