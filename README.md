# _Rust Final Project_

- _Justin Lin Sin Cho_
- _justin.linsincho@uoit.net_

## About the language

> - Rust first appeared in 2010
> - C/C++ offers more control while offering less safety, Python offers less control while offering more safety
> - Rust offers more control AND more safety
> - Rust closely resembles C/C++ in terms of code syntax

## About the syntax

> _The rust code syntax follows very similarly to C code syntax_

*Let form*

```rust
fn main(){
	let let_x = 5;
	let let_y = 9;

	println!("The sum of x and y is {}", let_x + let_y);
}
```

## About the tools

> _Describe the compiler or interpreter needed_.
> _To run rust, the compiler rustc is required._.
> _When a rust file is compiled with rustc, an executable file is produced_.
> _Ordinarily however, project files will be created and run using cargo, the rust package manager_.

> - cargo new hello-world
> - cargo build
> - cargo run

## About the standard library

> _Rust offers many functions and structures_

*String slice*

```rust
fn main(){
	let random_string = &"hello my name is Justin"[17..23];
	println!("his name is {}", random_string)
}
```

*struct form*

```rust
struct PersonDrink {
    age: i32,
    can_drink: bool,
}

fn main() {
    let richard = PersonDrink { age: 4, can_drink: false}; 

    println!("Richard is {} years old and it is {} that he can drink", richard.age, richard.can_drink);
}
```

## About open source library

> _A team from Mozilla is currently working on an experimental browser engine called Servo which is written in rust._
> _The goals of Servo is to create a project with better parallelism, security, modularity and performance._
> _More information about Servo can be found at https://servo.org/_


# Analysis of the language

> _Organize your report according to the project description
document_.

> _Rust is a procedural language similar to C++ but does have some features found in functional programming. For example, all variables created in rust is by default immutable however they can be set to be mutable on creation. An entire subreddit has been created dedicated to rust and an interesting post found in regards to rust from the point of view of a clojure programmer can be found at https://www.reddit.com/r/rust/comments/868suv/rust_first_impression_from_clojure_developer/._

> _Rust does support meta-programming such as macros. In Rust syntax, all macros will look like function calls with "!" following the name"._

*Macro creation*

```rust
macro_rules! new_macro {
	() => (
		println!("I am not a useful macro!");
	)
}

fn main(){
	new_macro!();
}
```

> _Rust uses lexical scoping and when compiling code and is a statically typed programming language. This can be verified by attempting to compile the below code. A compilation error will occur stating binary operation '+' cannot be applied to type '&str' proving that variable types are determined on compilation._

```rust
fn main() {
    let e = "sdfsdf";
    let m = 352;

    let c = e+m;

}
```

> _Rust is appears to have many advantages over other types of code while not too many disadvantages. One advantage of rust over C++ is that rust is much safer when dealing with concurrency and multiple sources attemping to access a single source of data with the concept of ownership. One advantage rust has over languages such as Java is that it runs much faster than Java. A disadvantage of rust that I personally found is the lack of GUI support in comparison to Java where creating GUIs can be very simple with the use of Jframes._

