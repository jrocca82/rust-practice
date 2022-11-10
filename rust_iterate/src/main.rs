fn main() {
    // If/Else
    let first = false;
    let second = true;
    let name = "Jojo";
    let name_len = name.len();
    
    if first && second {
        println!("Both are true");
    } else if first || second {
        println!("At least one of them is true");
    } else {
        println!("None of them");
    }

    let message = if name_len == 4 { "Four" } else { "Not Four" };

    // Match statements
    match second {
        true => {
            println!("True");
        }
        false => {
            println!("False");
        }
    }

    match name_len {
        4 => { println!("Four"); }
        5 => { println!("Five"); }
        6 => { println!("Six"); }
        _ => { println!("Different"); }
    }

    let message2 = match name_len {
        4 => { "Four" }
        5 => { "Five" }
        6 => { "Six" }
        _ => { "Different" }
    };
 
    println!("{}, {}", message, message2);

    // Ranges
    if name_len <= 5 {
        println!("Short");
    } else if name_len >= 6 && name_len < 11  {
        println!("Medium");
    } else {
        println!("Long");
    }
 
    match name_len {
       0..=5  => { println!("Short"); }
       6..=10 => { println!("Medium"); }
       _ => { println!("Long"); }
    }

    // Loops
    // Print the first n positive integers
    let n = 10;
    let mut i = 1;
 
    loop {
        println!("loop: {}", i);  // statements inside the loop
        i = i + 1;                // increment
        if i > n {                // terminal condition
            break;
        }        
    }

    // Need to reset i to run another loop
    println!("in-between loops: {}", i);
    i = 1;

    while i <= n{             // terminal condition
        println!("while: {}", i); // statements inside the loop
        i = i + 1;                // increment
        if i > n {                // terminal condition
            break;
        }
    }

    println!("in-between loops: {}", i);

    // For loops
    for i in 1..=10 {             // increment and terminal condition
        println!("for: {}", i);   // statements inside the loop
    }

    let cheat_code = [19, 65, 9, 17];

    for i in 0..cheat_code.len() {
        println!("cheat code: {}", cheat_code[i]);
    }
 
    for value in cheat_code {
        println!("cheat code by value: {}", value);
    }

}