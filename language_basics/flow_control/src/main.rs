fn main() {

    example_if();
    // example_match();
    // example_loops();
    // example_loops_break_continue();
}

fn example_if() {

    let age = 36;
    // if-condition
    if age > 35 {
        println!("You are old");
    }

    // if-else condition
    let height = 1.8;
    if height > 1.87 {
        println!("You are tall");
    }
    else {
        println!("You are not so tall");
    }

    // if-else if-else condition
    let temp = 35;
    if temp <= 10 {
        println!("It's cold!");
    } else if temp <= 25 {
        println!("It's nice");
    } else if temp <= 30 {
        println!("It's warm");
    } else {
        println!("It's hot!")
    }

    let message = if age > 35 {"Hi buira"} else {"Hi kochi"};
    println!("{}", message); 
}

fn example_match() {

    let num = 100;

    // exact match
    println!("\nUsing a match to test an expression against patterns");
    match num {
        100 => println!("A hundred"),
        200 => println!("Two hundred"),
        _   => println!("Who cares")
    }

    // match witin a range(including upper bound)
    match num {
        25..=50  => println!("25 to 50"),
        51..=100 => println!("51 to 100"),
        _        => println!("Who cares")
    }

    // multiple match with OR
    match num {
        25 | 50 | 75  => println!("25, 50, or 75"),
        100 | 200     => println!("100 or 200"),
        _             => println!("Who cares")
    }

    // conditional match
    match num {
        x if x < 50  => println!("Less than 50"),
        x if x == 75 => println!("75"),
        _             => println!("Who cares (could be 100 maybe)")
    }
}

fn example_loops() {

    // println!("\nUsing an infinite loop");
    // loop {
    //     println!("This loop will go on forever. Hit Ctrl-C to stop me!");
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // }

    // while loop
    println!("\nUsing a while loop");
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    // for loop over a range
    println!("\nUsing a for loop over a range (inclusive lower bound, exclusive upper bound)");
    for i in 0..10 {
        println!("{}", i);
    }

    // for loop over a range including upper bound
    println!("\nUsing a for loop over a range (inclusive lower bound, inclusive upper bound)");
    for i in 0..=10 {
        println!("{}", i);
    }

    // for loop iterating over an array
    println!("\nUsing a for loop over an array");
    let arr = [99, 55, 95, 100, 82];
    for elem in arr {
        println!("{}", elem);
    }
}

fn example_loops_break_continue() {

    println!("\nexample using break and continue");
    
    let arr = [99, 45, 85, 100, 82];
    for elem in arr {
        if elem == 100 {
            println!("Found 100, so break out of loop completely");
            break;
        } 
        if elem < 50 {
            println!("Found value less than 50, continue to next iteration");
            continue;
        }
        println!("{}", elem);
    }

    'outer: loop {
        println!("Entered the outer loop");
        loop {
            println!("Entered the inner loop");
            break 'outer;  // Break the outer loop.
        }
        // println!("This point will never be reached in this example");
    }
    println!("Exited the outer loop");

    println!("The End!");
}