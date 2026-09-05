 fn main() {
 	let t:f64 = 450_000.00;
 	let m:f64 = 1_500_000.00;
 	let h:f64 = 750_000.00;
 	let d:f64 = 2_850_000.00;
 	let a:f64 = 250_000.0;

 	//q=quantity
 	let qt:f64 = 2.0;
 	let qm:f64 = 1.0;
 	let qh:f64 = 3.0;
 	let qd:f64 = 3.0;
 	let qa:f64 = 1.0;

 	let qty = qt + qm + qh + qd + qa;
 	let sum = (qt*t) + (qm*m) + (qh*h) + (qd*d) + (qa*a);
 	let avg = sum/qty;

 	println!("The sum is {} and the average is {}",sum,avg);
}