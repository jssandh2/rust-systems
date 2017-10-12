fn main() {
    // `Vector` : A "dynamically allocated" array (Cons of a Type, recursively added => One Type, Variable Size)
    // *Always* allocate data on the {Heap} in continuous memory increments -> Initiated by the `vec!` macro

    let v: Vec<i32> = vec![1, 2, 3, 4, 5]; // v : vec<i32, 5>
    let mut w = vec![0; 10]; // [0,0,0...,0], v : vec<i32, 5>

    // If the size of the allocation is in question, invoke `Box` and *not* `Vector`
    let x = Box::new(5);   // This is closer to the Type-Theoretic defn of `List<T>` := Nil | Cons(T, Box(List<T>))
    // A box _is_ the type of a pointer, and therefeore, allows you to specify a think reference to memory without
    // an explcit declaration of size at compile time

    
    // Access Elements -> Vector
    let a: i32 = v[1];
    println!("The second element of the vector is: {}", a);

    // Indexing _only_ works with the `usize` type (not a signed representation)
    let i: usize = 0;
    v[i];

    // To handle certain run-time errors, one can use the `get()`, `set()` methods with the Mondadic Option-Type
    match v.get(3) {
        Some(x) => println!("The 3rd item in the array is: {}", x),
        None => println!("Sorry, either don't fetch negative indices, or indices longer than the vector.")
    }

    // `Iterator` over (vectors)
    // _Only_ mutable references can allow for multiple iterations
    // Iterating without a mutable reference relinquishes ownership after
    // a single iteration
    for i in &mut w {
        println!("A mutable reference: {} to an element in v", i);
    }

    for i in w {
        println!("Ownership transfer for the element: {}, in v", i);
    }
}
