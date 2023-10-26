fn main() {
	let t:f32 = 900_000.00;
	let m:f32 = 1_500_000.00;
	let h:f32 = 2_250_000.00;
	let d:f32 = 8_550_000.00;
	let a:f32 = 250_000.00;


	// average
	let sum = t + m + h + d + a;
	let av = sum / 10.0;
	println!("The average is {}",av);
	println!("The sum is {}",sum);
}