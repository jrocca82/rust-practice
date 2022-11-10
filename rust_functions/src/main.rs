fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn cel_to_far(c: f64) -> f64 {
    (c * 9.0/5.0) + 32.0
}

fn far_to_cel(f: f64) -> f64 {
    (f - 32.0)* 5.0/9.0
}

// 2. Generate the nth Fibonacci number.
fn get_fib(n: i32) -> i32 {
    (n-1) + (n-2)
}
 
fn main() {
    let sum = sum(5, 4);
    println!("{}", sum);

    let celsius1 = 37.0;
    let c1 = cel_to_far(celsius1);
    let celsius2 = 0.0;
    let c2 = cel_to_far(celsius2);
    let farenheit1 = 32.0;
    let f1 = far_to_cel(farenheit1);
    let farenheit2 = 104.0;
    let f2 = far_to_cel(farenheit2);
 
    println!("{} C = {} F", celsius1, c1);
    println!("{} C = {} F", celsius2, c2);

    println!("{} F = {} C", farenheit1, f1);
    println!("{} C = {} F", farenheit2, f2);

    let nthfib = get_fib(15);
    println!("The 15th fibonacci number is {}", nthfib);
}
