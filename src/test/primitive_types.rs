fn main() {
    // Boolean
    let x = true;
    let y: bool = false;

    // Char
    let x = 'x';
    
    // Numeric Types
    let x = 42; // `x` has type `i32`
    let y = 1.0; // `y` has type `f64`

    // [Fixed-Size] Integers (signed) -> `i8`, `i16`, `i32`, `i64` : [-2^{n/2}, 2^{n/2} - 1]  --> in
    // [Fixed-Size] Integers (unsigned) -> `u8`, `u16`, `u32`, `u64` : [0, 2^{n} - 1]  --> un
    // [Variable-Size] Integers (signed), (unsigned) --> isize, usize
    // [Fixed-Size] (signed) -> `f32`, `f64`

    // Arrays
    let a = [1, 2, 3]; // a : [i32; 3]
    let mut m = [1, 2, 3]; // m : [i32; 3]

    let a = [0; 20]; // a = [0, 0, 0, 0 ... , 0], a : [i32; 20]
    println!("Array a has : {} elements, and the 2nd element is: {}", a.len(), a[1]);

    let names = ["Juspreet", "Cook", "Levin"]; // names : [&str; 3]
}
