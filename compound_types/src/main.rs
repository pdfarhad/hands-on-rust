fn main() {
    // example_arrays();
    // example_arrays_bound_checking(1);
    // example_arrays_techniques();
    example_tuples();
}

fn example_arrays() {
    // We can create an array using simple literal syntax.
    let array1 = [123, 556, 7];
    println!("array1 is {:?}, array1 length is {}, first element is {}", array1, array1.len(), array1[0]);

    // We can also create a mutable array - We can change items, but We can't change the size.
    let mut array2 = [67, 88, 676, 90];
    array2[0] = 999;
    println!("array2 is {:?}", array2);
}

fn example_arrays_bound_checking(index: usize) {
    let a = [765, 989, 345];
    println!("\nElement {} is {}", index, a[index]);
}

fn example_arrays_techniques() {
    // We can specify type info and size.
    let array1: [i64; 5];
    array1 = [77, 66, 55, 44, 33];
    println!("\narray1 is {:?}", array1); 

    // We can fill an array with [filler;size] syntax. 
    let array2 = [101; 5];
    println!("array2 is {:?}", array2); 
}

fn example_tuples() {
    // A tuple is a fixed-size heterogeneous collection.
    let tupple1 = (456, String::from("Bangladesh"), 3.14);
    println!("\ntupple1 is {:?}, individual elements are {}, {}, {}", tupple1, tupple1.0, tupple1.1, tupple1.2);

    // We can also create a mutable tuple (but We have to be consistent with element types).
    let mut tupple2 = (100, String::from("Bangladesh"), 3.14);
    tupple2.0 = 199;
    println!("tupple2 is {:?}", tupple2);

    // We can specify type info.
    let tupple3: (i32, String, f64);
    tupple3 = (200, String::from("Dhaka"), 45.99);
    println!("tupple3 is {:?}", tupple3);

    // We can also have an empty tuple (handy for functions that return nothing at all).
    let tupple4 = ();
    println!("tupple4 is {:?}", tupple4);
}