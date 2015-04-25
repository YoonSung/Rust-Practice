fn main() {
	print_number(5);
	print_sum(10, 2);
}

fn print_sum(x: i32, y: i32) {
	println!("sum is : {}", x);
}

fn print_number(x: i32) {
	println!("x is: {}", x);
}
	
fn variable_binding() {
	//let x: i32; //warning not used
	//println(!"The value of x is : {}", x); //error not initialized
    println!("Hello world!");
}
