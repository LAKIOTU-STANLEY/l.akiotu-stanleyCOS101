fn main() {
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	let _a:f64 = p * (1.0 - (r/100.0)).powf(t);
	println!("The amount after depreciation is {}",_a );
}