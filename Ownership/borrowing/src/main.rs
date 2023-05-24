fn main() {
    example_syntax_implicit_typing();
	example_syntax_explicit_typing();
	example_borrow_checkereference1();
	example_borrow_checkereference2();
}

fn example_syntax_implicit_typing() {
	
	let s = String::from("hello");
	
	let reference1 = &s;
	let reference2 = &s;
	
	println!("{} {} {}", s, reference1, reference2);
}

fn example_syntax_explicit_typing() {
	
	let string1: String = String::from("hello");
	
	let string2: &String = &string1;
	let string3: &String = &string1;
	
	println!("{} {} {}", string1, string2, string3);
}

fn example_borrow_checkereference1() {
	
	let mut s = String::from("hello");
	
	let reference1 = &mut s;
	reference1.push_str(" Bangladesh");

	// This won't compile! Can't have more than one mutable borrow.
	// let reference2 = &mut s;
	// reference2.push_str(" and goodbye");

	println!("{}", reference1);
}

fn example_borrow_checkereference2() {
	
	let mut s = String::from("hello");
	
	let reference1 = &s;
	
	// This won't compile! Can't borrow as mutable if already borrowed as immutable.
	// let reference2 = &mut s;
	// reference2.push_str(" world");	

	println!("{}", reference1);
}
