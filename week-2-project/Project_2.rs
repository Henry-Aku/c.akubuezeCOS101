fn main(){
	let t:f64 = 450000.0;
	let m:f64 = 1500000.0;
	let h:f64 = 750000.0;
	let d:f64 = 2850000.0;
	let a:f64 = 250000.0;

	let sum:f64 = t + m + h + d + a;

	let average:f64 = sum/5.0;

	println!("Your sum is: ₦{}", sum);
	println!("Your average is: ₦{:2}", average);
}