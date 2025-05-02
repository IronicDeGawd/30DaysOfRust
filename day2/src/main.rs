//STACK : Stack is used to store smaller values and are easier to access [Static Memory Allocation]
//HEAP : Heap is used to store bigger values but are difficult to access [Dynamic Memory Allocation]

/* For strings, The reference to strings are stored in stack but the actual
string is stored in heap for dynamic sizing and storage */

/* For integers, char etc, the value is stored on the stack */

fn ownership() {
    /* Adding underscore tells rust compiler to stop checking if this variable will be used anywhere or not */
    let _s1 = String::from("Hello rust!");

    /* Ownership in rust is used to assign memory to variables for its values and provide its ownership. This helps to ensure memory safety and prevent memory & pointer errors */

    /* This moves ownership of the string from _s1 to _s2. The pointer _s1 in stack to the heap value is removed and _s1 is no longer valid after this line. This is called Ownership Transfer. */
    let _s2 = _s1;

    /* This create a string slice from 0 to 5th index of _s2 of type &str without taking the ownership of the original _s2. */
    let _s2_slice: &str = &_s2[0..=5];

    /* This line when uncommented would give the error - "value borrowed here after move" */
    // println!("Value of _s1 {}", _s1);

    println!("Value of _s2 {}", _s2);
    println!("Value of _s2_slice {}", _s2_slice);
}

fn borrowing() {
    /* COPY vs NON-COPY:
     - Copy trait: Types like numbers (i32, u8, etc), booleans, and chars are Copy types.
       When assigned, they are duplicated automatically without ownership transfer.

     - Non-Copy trait: Types like String, Vec, etc. are Non-Copy (they implement Move).
       When assigned, ownership transfers and the original variable becomes invalid.
    */

    /* Borrowing allows owning a value using references without changing ownership */
    /* References are of two types -  Mutable Reference and Immutable Reference */

    let my_str: String = String::from("Immutable Reference");
    let _my_ref: &String = &my_str;
    let _second_ref: &String = &my_str;

    /* We can have multiple immutable references to a variable */
    /* IMPORTANT : We cannot have immutable & mutable reference for a variable at same time */

    /* We can only have ONE mutable reference */
    let mut x: i16 = 707;
    let _x_ref: &mut i16 = &mut x;

    /* Uncommenting this code will give the error: "cannot borrow `x` as mutable more than once at a time" */
    //let x_ref2: &mut i16 = &mut x;
    //println!("Value of first ref is {} and second ref is {}", _x_ref, x_ref2 );

    let mut hello = String::from("Hello");
    println!("Value of hello variable is {}", hello);

    /* Passing the mutable reference to the function */
    add_world(&mut hello);
    println!("Value of hello variable is {}", hello);

    /* Creating a function to use mutable ref to change the value of mutable variable */
    fn add_world(hello: &mut String) {
        hello.push_str(" world");
    }

    /* To a move ownership of a String, all its references should be freed from memory to prevent dangling pointer errors */
    let a: String = String::from("hello");
    let _a_ref: &String = &a;

    /* This line uncommented will give error : "cannot move out of `a` because it is borrowed" */
    // let b: String = a;
    // println!("Value of b is {}", b);

    println!("Value of a ref is {}", _a_ref);
}

fn main() {
    ownership();
    borrowing()
}
