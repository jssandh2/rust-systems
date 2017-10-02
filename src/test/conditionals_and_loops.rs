fn main() {
    // Conditionals
    let x: i32 = 5;

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six!");
    }

    // Expression-based statements inside a conditional workflow should be written more concisely
    let y = if x == 5 { 10 } else { 15 }; // y : i32

    // Loops --> `loop`, `while`, `for`

    // 1) `loop` --> Better suited to infinite loops than `while true`, as it is more explicit to the compiler
    //    loop {
    //        println!("Loop forever!");
    //    }

    // 2) `while`
    let mut z: i32 = 5;
    let mut done = false;

    while !done {
        z += z - 3;

        println!("{}", z);

        if z % 5 == 0 {
            done = true;
        }
    }

    // 3) `for`
    // general syntax : for var in expressions { code }
    
    // Without index enumeration
    for i in 0..10 {
        println!("Enumerated Value : {}", i);
    }

    // With index enumeration
    for (index, value) in (5..10).enumerate() {
        println!("index : {} , value : {}", index, value);
    }
    
    // 4) Iterators
    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("Linenumber : {}, Line : {}", linenumber, line);
    }

    // `break` --> Terminates the entire loop and goes to the first instruction beyond the loop
    // `continue` --> Goes to the next iteration in the loop, skipping any instructions from : {continue, {loopstart})
    let mut e = 15;
    
    loop {
        e += e - 3;

        if x % 5 == 0 { break; }
    }

    for z in 0..15 {
        if z % 2 == 0 { continue; }

        println!("z : {}", z);
    }

    // ('label) : Loops that are labelled
    'outer : for x in 0..10 {
        'inner : for y in 0..10 {
            if x % 3 == 0 { continue 'outer; } // continues the loop over x => All iterations of `y` are skipped
            if y % 3 == 0 { continue 'inner; } // continues the loop over y => Any inner work is skipped
            println!("x :  {}, y : {}", x, y);
        }
    }
}

