fn main() {
    example_scope();
	// example_string_type();
	// example_copying_vs_moving();
	// example_clone();
	
	// let x = example_scope();
	// println!("x {}", x);

}

fn example_scope() -> i32 {
	
	let x = 99;

	if x != 0 {
		let s = "Farhad";
		println!("s {}", s);
	}
	x
	// Nope:
	// println!("s {}", s);
}

fn example_string_type() -> String{
	
	let mut s = String::from("Hello");
	s.push_str(" Bangladesh");
	println!("s {}", s);
	s
}   // s goes out of scope here, so drop() is called on the String s (because String implements the Drop trait).

fn example_copying_vs_moving() {
	
	// Simple types implement the Copy trait. 
	// When you assign, it copies the value.
	let x = 42;
	let y = x;
	println!("x {}, y {}", x, y);
	
	// Other types don't implement the Copy trait. 
	// When you assign, it moves the value (i.e. transfers ownership). The original is invalidated.
	let string1 = String::from("hello");
	let string2 = string1;
	
	// Nope! Can't use string1 because its value has been moved into string2.
	// println!("string1 {}", string1);

	// This is ok.
	println!("string2 {}", string2);

}   // string2 goes out of scope here, so drop() is called on the String string2. 


fn example_clone() {

	// Simple types implement the Copy trait
	let x = 42;
	let y = x;
	println!("x {}, y {}", x, y);

	// Other types don't implement the Copy trait.
	// If you do want to copy without invalidating the original, call clone().
	let mut string1 = String::from("Hello");
	let string2 = string1.clone();

	string1.push_str(" Bangladesh, Habijabi");
	println!("string1 {}, string2 {}", string1, string2);
}
