fn main() {
    example_integers();
    // example_floats();
    // example_other_simple_types();
    // example_techniques();
}

fn example_integers() {

    // Rust has signed integer types i8, i16, i32, i64, i128.
    let a1: i32 = -98765;          
    let a2: i32 = 0xFFFE;
    let a3: i32 = 0o144;
    let a4: i32 = 0b1101;

    // Rust has unsigned integer types u8, u16, u32, u64, u128.
    let b: u32 = 12345;         

    // Rust has architecture-specific integer types isize, usize.
    let c:isize = 24680;         
    
    println!("\nNumbers are {} {} {} {} {} {}", a1, a2, a3, a4, b, c);
    println!("Numbers in reverse order are {5} {4} {0} {2} {1} {3}", a1, a2, a3, a4, b, c);
    println!("isize is {} bytes on my machine", std::mem::size_of::<isize>());
}

fn example_floats() {
    // Rust has single-precision and double-precision floats.
    let f1: f32 = 1.23456;
    let f2: f64 = 9.87654;
    
    println!("\nFloats are {} {}", f1, f2);
    
}

fn example_other_simple_types() {

    let is_coder: bool = true;
    let can_sing: bool = false;

    println!("\nAm I a Coder? {}, can I sing? {}", is_coder, can_sing);

    let favourite_emoji = '😎';
    println!("My fav emoji is {}", favourite_emoji);
} 

fn example_techniques() {

    // Rust can infer types.
    let a = -9876;
    let pi = 3.14;
    let c = 'X';
    println!("\na is {}, pi is {}, c is {}", a, pi, c);

    // Variables are immutable by default.
    let d = 0;
    //d = 1;
    println!("d is {}", d);

    // Must use the mut qualifier to make a variable mutable.
    let mut e = 0;
    println!("e originally is {}", e);
    e = 1;
    println!("e after mutating is {}", e);
 
    // If you don't use a variable, prefix name with _ to avoid compiler warning.
    let _f = 0;

    // You can cast using the "as" keyword.
    let g = 3.99;
    let h = g as i32;
    println!("g is {}, h is {}", g, h);

    // Rust enables you to redeclare a variable in the current scope. This is called shadowing and it's quite cool.
    let num = String::from("12345");
    println!("num as a string is {}", num);
    let num = 12345;
    println!("num+1 as a number is {}", num + 1);

    // You can define compile-time constants. You must specify the type btw.
    const SECONDS_IN_HOUR: i32 = 3_600;
    const SECONDS_IN_DAY: i32 = 24 * SECONDS_IN_HOUR;
    println!("There are {} seconds in a day", SECONDS_IN_DAY);  
}