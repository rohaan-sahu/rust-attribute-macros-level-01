use my_macros::{
	log_call,
	log_call_comp,
	double_output,
	multiply,
	log_multiply,
	describe,
	builder
};

#[log_call]
#[double_output]
fn add(a: i32, b: i32)-> i32 {
	a + b
}

#[log_call_comp]
#[multiply(10)]
fn mul(x: i32, y: i32)-> i32{
	x * y
}

#[double_output]
fn double_it(m: i32)-> i32{
	m
}

#[multiply(11)]
fn mul_eleven(n: i32)-> i32{
	n
}

#[log_multiply(factor=3, log=true)]
fn log_mul_test(s: i32)->i32 {
	s
}

#[describe]
struct Point {
	x: i32,
	y: i32,
	z: i32,
}

#[describe]
#[builder]
#[derive(Debug)]
struct Person {
	name: String,
	age: u32,
	active: bool
}

#[describe]
struct Maker;

#[describe]
struct NewPoint(i32, i32);

fn main() {
	let x = add(2,3);
	println!("result {}",x);

	let m = mul(3,4);
	println!("product: {}",m);

	let d = double_it(4);
	println!("double: {}",d);

	let n = mul_eleven(12);
	println!("multiplyed by eleven: {}",n);

	let s = log_mul_test(5);
	println!("product logged: {}",s);

	let p = Point{x: 1, y: 2, z: 3};
	p.describe();

	let a = Person {name: "Rhn".to_string(), age: 4, active: true};
	let b = Maker;
	let c= NewPoint(2,3);

	a.describe();
	b.describe();
	c.describe();

	let per = Person::new("Alice".to_string(),30,true);
	per.display();
}
