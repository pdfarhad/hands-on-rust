fn main() {
    println!("Hello, world!");
    example_closures();
}

fn example_closures() {
    let sqr_b = |i: i32| -> i32 { i * i };
    let sqr_c = |i| i * i;

    println!("sqr_b result {}", sqr_b(3));
    println!("sqr_c result {}", sqr_c(5));
}