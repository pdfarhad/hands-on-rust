use std::collections::HashMap;

fn main() {
    example_vec();
    example_vec_bounds_checking(0);
    example_hash_map();
}

fn example_vec() {

    println!("Using a vector");
    
    let mut vec1: Vec<i32> = Vec::new();
	
    // Note, could rewrite the above as:
    let mut _vec1b = Vec::<i32>::new();
	
    vec1.push(99);
    vec1.push(101);
    vec1.push(100);
    println!("vec1 is {:?}, length is {}, first element is {}", vec1, vec1.len(), vec1[0]);

    let mut vec2 = vec!["Dhaka", "Barisal", "Khulna"];
    vec2.insert(0, "Rajshahi");
    println!("{:?}", vec2);

    for item in vec2 {
        println!("{}", item)
    }
}

fn example_vec_bounds_checking(index: usize) {
    
    println!("\nVector bounds-checking");
    
    let vec= [100, 99, 98];
    
    // This could cause the program to panic (i.e. crash).
    // let item = v[index];

    // Better to call get(), which returns an Option<T>.
    // Option is an enum that either contains Some<T> or None.
    let opt = vec.get(index);
    match opt {
        Some(value) => println!("At index {}, found value {}", index, value),
        None        => println!("At index {}, no value found", index)
    }

    println!("If you KNOW an Option isn't None, you can unwrap its value. Let's try... {}", opt.unwrap())
}

fn example_hash_map() {

    println!("\nUsing a HashMap");
    
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert(String::from("BD"), 88);
    m.insert(String::from("IND"), 91);
    m.insert(String::from("PAK"), 92);
    println!("m is {:?}, length is {}", m, m.len());

    println!("If you only want to insert if doesn't already exist...");
    let val = m.entry(String::from("UK")).or_insert(44);
    println!("Value for UK is {}", val);

    // This would cause the program to panic (i.e. crash).
    // let val = m["RUS"];
    // println!("Value for RUS is {}", val);

    // Better to call get(), which returns an Option<T>.
    // Option is an enum that either contains Some<T> or None.
    let opt = m.get("RUS");
    match opt {
        Some(val) => println!("Value for RUS is {}", val),
        None      => println!("No value found for RUS")
    }
}
