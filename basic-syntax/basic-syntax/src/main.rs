//this rust code shows much of the syntax used. Functions with and without return variables, loops,
//if statements, type casting, mutable and immutable variables...


//show numbers between 0 and x that are smaller and even
fn even_and_smaller(x:f32){
	let mut z = 0;

	let y = x.floor() as i32;

	loop {
		z = z+1;
		if z>=y{
			break;
		}else if z%2==0 {
			println!("{0} is even and smaller than {1}",z, y);
			continue;
		}
	}
}

//return the addition of x and y
fn addition(x:f32, y:f32) -> f32{
	x+y
}


//if x is greater than 20, return true
fn greater_than_twenty(x:f32) ->bool{
	if x>20.0 {
		true
	}else{
		false
	}
}

fn main() {
    let a = 7.2523;
    let b = 9.2434;

    let c = addition(a,b);

    println!("the sum of {0} and {1} is {2}", a, b, c);

    let big_number = greater_than_twenty(c);

    println!("It is {0} that {1} is a big number", big_number, c);

    even_and_smaller(c);
}
