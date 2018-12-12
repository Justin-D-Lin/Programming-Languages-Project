//creates a vector containing 20 random numbers ranging from 0-100. Prints the unsorted vector then sorts and reprints. 

extern crate rand;
use rand::Rng;

fn main() {
    let mut vec = Vec::new();
    for _x in 0..20{
    	let num = rand::thread_rng().gen_range(0, 100); 
    	vec.push(num);
    }

    println!("Unsorted vector:");
    for _x in vec.iter(){
    	println!("> {}", _x);
    }
    println!("=================================");
    println!("Sorted vector:");

    vec.sort();

    for _x in vec.iter(){
    	println!("> {}", _x);
    }
}
