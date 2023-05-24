// To suppress warnings about unused enum variants. We can annotate variants with #[allow(dead_code)] 

fn main() {
    demo_simple_enums();
    // demo_enum_with_data();
    // demo_using_option_enum();
    // demo_using_result_enum(String::from("98765"));
}

enum TrafficLight {
    Red,
    Green,
    Yellow
}

fn demo_simple_enums() {

    println!("Demo simple enums");

    let c: TrafficLight = TrafficLight::Red;   
    match c {
        TrafficLight::Red   => println!("Laal"),
        TrafficLight::Green => println!("Sobuj"),
        TrafficLight::Yellow  => println!("Lolo")
    }
}

// enum SAddress {
//     Number(i32),
//     SNumber(i32),
//     Truth(bool),
//     Name(String),
//     Unknown
// }

enum Address {
    Number(i32),
    Name(String),
    Unknown
}


fn demo_enum_with_data() {

    println!("\nDemo enums with data");

    let h: Address = Address::Number(4);
    match h {
        Address::Number(n) => println!("You live in house number {}", n),
        Address::Name(s)   => println!("You live in a house named {}", s),
        Address::Unknown   => println!("Your house location is unknown"),
    }
    println!("Btw the size of Address is {} bytes", std::mem::size_of::<Address>());
    // println!("Btw the size of SAddress is {} bytes", std::mem::size_of::<SAddress>());
    // println!("Btw the size of SAddress is {} bytes", std::mem::size_of::<bool>());

}

fn demo_using_option_enum() {
    
    println!("\nDemo using the Option<T> enum");
    
    // Rust defines a standard enum Option<T> as follows:
    // enum Option<T> {
    //    Some(T),
    //    None
    // }

    let favnum: Option<i32>;

    // Uncomment one of the following statements.
    favnum = Option::Some(107);
    // favnum = Option::None;

    match favnum {
        Some(n) => println!("My favourite number is {}, good choice", n),
        None    => println!(" What The Fuchka????!")
    }

    // We can use unwrap_or() to extract Some value from an Option, or use a fallback value if None.
    // println!("My fav num  name is {}", favnum.unwrap_or(42));
}

fn demo_using_result_enum(s: String) {
    
    println!("\nDemo using the Result<T, E> enum");

    // Rust defines a standard enum Result<T, Err> as follows:
    // enum Result<T, E> {
    //    Ok(T),
    //    Err(E)
    // }
   
    match s.parse::<i32>() {
        Ok(v)  => println!("Parsed String as i32: {}", v),
        Err(e) => println!("Error parsing String as i32: {}", e),
    }

    // You can use unwrap_or() to extract Ok value from a Result, or use a fallback value if Err.
    let good_str = String::from("1964");
    println!("Parsed String as i32: {}", good_str.parse::<i32>().unwrap_or(-1));

    // You can use expect() to specify a panic error message if Result is Err.
    // let bad_str = String::from("nineteen sixty four");
    // println!("Parsed String as i32: {}", bad_str.parse::<i32>().expect("Invalid!!"));
}
