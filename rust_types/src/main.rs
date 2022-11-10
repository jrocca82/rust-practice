fn main() {
    // Adding ints example
    let x = 2;
    let y = 5;
    println!("{} + {} = {}", x, y, x + y);

    // Sizing ints example
    let a: u8 = 255;
    let b: u16 = 65535;
    let c: u32 = 4294967295;
    let d: u64 = 18446744073709551615;

    println!("{}, {}, {}, {}", a, b, c, d);

    // uint min/max
    get_uint_values();

    // Signed int min/max
    get_int_values();

    // Floating points:
    let float = 0.1;
    let float_two = 0.2;

    println!("{}, {}, {}", float, float_two, float + float_two);

    //Re-ordering println!:
    println!("{1}, {0}, {2}, {1}", float, float_two, float + float_two);

    // Characters:
    let ch1 = "X";
    let ch2 = "\u{2603}";

    println!("{}, {}", ch1, ch2);

    // Booleans
    let on: bool = true;
    // Make mutable
    let mut off: bool = false;

    println!("{}, {}", on, off);

    off = !off;
    // Alternative mutability
    let on = !on;
    println!("{}, {}", on, off);

    // Immutable-- use all caps
    const IS_LAMP_ON: bool = false;
    println!("{}", IS_LAMP_ON);

    //Formatting macros
    let btc = "Bitcoin";
    let eth = "Ethereum";
    let sol = "Solana";
    let message = format!(
        "Kwargs: {solana}, {ethereum}, {bitcoin}",
        bitcoin = btc,
        ethereum = eth,
        solana = sol
    );
    println!("Look I've made this special {message}!", message = message);

    // Tuples
    let tuple = (100478, 133.8, "Solana", true);
    println!("{} {} {} {}", tuple.0, tuple.1, tuple.2, tuple.3);

    println!("{}, {:?}", tuple.0, tuple.2);

    println!("{}, {:#?}", tuple.0, tuple.2);

    // Arrays
    let cheat_code: [i32; 4] = [19, 65, 9, 17];
    println!("Array: {:?}", cheat_code);
    println!("First element of array: {}", cheat_code[0]);

    // Slice
    let slice = &cheat_code[1..3];
    println!("Slice: {:?}, length: {}", slice, slice.len());

    //Print 10 zeros
    let zeros = [0; 10];
    println!("Zeros: {:?}, length: {}", zeros, zeros.len());
}

fn get_uint_values() {
    let a_min: u8 = std::u8::MIN;
    let b_min: u16 = std::u16::MIN;
    let c_min: u32 = std::u32::MIN;
    let d_min: u64 = std::u64::MIN;
    let e_min: usize = std::usize::MIN;

    let a_max: u8 = std::u8::MAX;
    let b_max: u16 = std::u16::MAX;
    let c_max: u32 = std::u32::MAX;
    let d_max: u64 = std::u64::MAX;
    let e_max: usize = std::usize::MAX;

    println!("{}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("{}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);
}

fn get_int_values() {
    let a_min: i8 = std::i8::MIN;
    let b_min: i16 = std::i16::MIN;
    let c_min: i32 = std::i32::MIN;
    let d_min: i64 = std::i64::MIN;
    let e_min: isize = std::isize::MIN;

    let a_max: i8 = std::i8::MAX;
    let b_max: i16 = std::i16::MAX;
    let c_max: i32 = std::i32::MAX;
    let d_max: i64 = std::i64::MAX;
    let e_max: isize = std::isize::MAX;

    println!("{}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("{}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);
}