#[macro_use]
extern crate text_io;

fn main() {
    use std::io::{stdin};
	let mut operation = String::new();

	println!("What operation would you like to complete?");
	println!("(add, subtract, divide, multiply, power)");

	stdin().read_line(&mut operation).expect("Failed to read line");

	println!("provide two numbers");

	let x: f32 = read!();
	let y: f32 = read!();

	if operation.trim() == "add" {
	    println!("the sum of {0} and {1} is {2}",x , y, x+y);
	}else if operation.trim() == "subtract"{
		println!("the difference of {0} and {1} is {2}",x , y, x-y);
	}else if operation.trim() == "divide"{
		println!("the quotient of {0} and {1} is {2}",x , y, x/y);
	}else if operation.trim() == "multiply"{
		println!("the product of {0} and {1} is {2}",x , y, x*y);
	}else if operation.trim() == "power"{
		println!("the result of {0} to the power of {1} is {2}",x , y, x.powf(y));
	}
}