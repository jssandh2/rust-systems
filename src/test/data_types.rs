fn main(){
   let x = 5; // Binding to _bind_ x to 5. Rust has Type Inference.
   let (y, z) = (1, 2); // Binding that uses _patterns_
   let k: i32 = 5; // Binding to _bind_ k to 5, with a static "int 32" type
   
   /* By _default_ , bindings are _immutable_ => Shadowing may occur, but we
      are in the realm of Strong Types (sort of). We can _not_ mutate types.
      NOTE : To mutate them, there must exist no binding.
   */
   // Mutable binding --> 'mut'
   let mut xp = 5;
   xp = 7;
   println!("The value of x is: {}", xp); // This should print : 7

   /* Unused bindings can be non-initialized. However, bindings that are used (in I/O or otherwise)
      will lead to an error.
   */
   // let x: i32; println!("The value of x is: {}", x); --> Leads to a compile-time error.
   // let x: i32; println!("Hello, World!"); --> Leads to a compile-time warning, but no error.
   
   /* Variable bindings are enclosed lexically by blocks --> { .. } - Addressing them outside that
      scope on a popped stack leads to them not being found in the environment. 
   */
   /* Blocks are recursively nested. Scoping into the stack works, but referencing popped scopes
      does not However, the inverse does not.
   */
   {
       let y: i32 = 6;
       println!("The value of x is : {}, and the value of y is : {}", x, y);     
   }
    
   // Shadowing <--> Immutability aren't always interchangeable. Imm can lock mut via shadowing.
   let mut a: i32 = 8;
   a = 10;
   let a = a; // a is now _immutable_ and *locked* to 10 (util shadowed).

  // Mutability can also change types
  let e = 5;
  let e = "I am now a string!"; // e has changed types from i32 --> string
}
