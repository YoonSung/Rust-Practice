fn main() {
	//print_number(5);
	//print_sum(10, 2);
	let x:i32 = add_one(6);
	print_number(x);

	let x1: i32 = diverges();
	let x2: String = diverges();

	print!("test start");
	print!("{}", x1);
	print!("{}", x2);
}

fn add_one(x: i32) -> i32 {
	x + 1
}

fn print_sum(x: i32, y: i32) {
	println!("sum is : {}", x+y);
}

fn print_number(x: i32) {
	println!("x is: {}", x);
}
	
fn variable_binding() {
	//let x: i32; //warning not used
	//println(!"The value of x is : {}", x); //error not initialized
    println!("Hello world!");
}

fn diverges() -> ! {
	    panic!("This function never returns!");
}

