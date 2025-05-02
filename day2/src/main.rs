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


fn main(){
    ownership();
}
