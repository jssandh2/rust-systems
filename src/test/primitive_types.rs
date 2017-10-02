fn main() {
    // Boolean
    let x = true;
    let y: bool = false;

    // Char --> 1 Char = 4 bytes (not 1 byte)
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

    // Slicing --> Pointer to a beginning, with a length. Combination of : {[], &}
    // & --> Implies that a slice is similar to a Reference
    let b = [0, 1, 2, 3, 4];
    let b_duplicate = &b[..];  // A slice containing everything in `a`
    let middle = &b[1..4];     // A slice of `a` : [1, 2, 3]

    // str --> Unsized string type, that is useful when placed behind a reference `&` (`&str`)
    // Tuples
    let x = (1, "hello");
    let y: (i32, &str) = (1, "hello");

    let mut x = (1, 2);
    let y = (2, 3);
    x = y;   // A tuple is assigned to another (mutable) tuple, and type-check => The same Product-Type

    // Pattern-match on tuples
    let (x, y, z) = (1, 2, 3);
    println!("x is : {}, y  is : {}, z is : {}", x, y, z);

    let a = (0,); // Tuple of arity 1
    let b = (0);  // Normal zero

    // Indexing syntax
    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    // Functions --> Have arrow types
    fn foo(x: i32) -> i32 { x } // foo : i32 -> i32

    fn two_foo(x: i32, y: i32) -> i32 { x + y } // two_foo : (i32 * i32) -> i32
}
