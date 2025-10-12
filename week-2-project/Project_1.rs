fn main(){
	let p:f64 = 520000.0;
	let rate:f64 = 10.0;
	let years:f64 = 5.0;

	let amount:f64 = p * (1.0+ rate/100.0).powf(years);
	let compound_intrest:f64 = amount - p;

	println!("Compound Intrest: ₦{:2}", compound_intrest);

}