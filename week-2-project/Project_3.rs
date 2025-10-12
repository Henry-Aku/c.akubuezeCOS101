fn main(){
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	let amount:f64 = p * (1.0+ r/100.0).powf(n);
	let depreciation:f64 = p - amount;

	println!("Depreciation: ₦{:2}", depreciation); 
}