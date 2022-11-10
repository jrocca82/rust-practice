fn main() {
    // &str: string slice
    // "": string literal
    // String: mutable string

    let name: &str = "Jojo";
    let location: String = "Brisbane, Australia".to_string();
    let title: String = String::from("Blockchain Engineer");
 
    let title_slice = &title;
 
    println!("Hi, my name is {}.", name);
    println!("I live in {}.", location);
    println!("This name is {} characters long.", name.len());
    println!("My title is {}.", title);
    println!("Title as a string slice: {}.", title_slice);

    // Getting chars
    let digits = "0123456789".to_string();
 
    // 'c' - character (primitive type)
    // "c" - immutable string literal
    // Referencing a single character from a string
    if let Some(four) = &digits.chars().nth(4) {
        println!("Four: {}", four);
    } else {
        println!("Error");
    }

    // Concat
    println!(
        "String concatenation: {} {} {}",
        "first".to_string() + "second",
        format!("{}{}", "first", "second"),
        ["first", "second"].concat(),
    );
 
    let first = "1".to_string();
    let second = "2".to_string();
    let third = "3".to_string();
    let fourth = "4".to_string();
 
    println!(
        "String concatenation: {}",
        first + &second + &third + &fourth
    );
}
