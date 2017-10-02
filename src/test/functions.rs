fn print_sum(x: i32, y: i32) {
    println!("The sum of x : {} and y : {} is : {}", x, y, x + y); 
}

fn sum_number(x: i32, y: i32) -> i32 {
    x + y
}

fn sum_number_three(x: i32, y: i32, z:i32) -> i32 {
    x + y + z
}

// Divergent Types --> ! => \forall t \in T , t < !
fn diverges() -> ! {
    panic!("This function diverges!");
}

fn main(){
    let mut y = 5;
    let x = (y = 6); // `x` has the value `()`, _NOT_ 6

    let x: i32 = diverges();
    let x: String = diverges();

    // To turn verbose backtrace (stack) on and compile : RUST_BACKTRACE=(1,full) cargo (run,build)
    // Functions can have pointers too : Function pointers
    
    let f: fn(i32) -> i32;
    let f: fn(i32, i32) -> i32 = sum_number;

    let y = f(5, 6);
    
    print_sum(4, 8);
    let z = sum_number(5, 6);
    diverges();
    print_sum(f(5, 6), f(8, 9)); 
}
