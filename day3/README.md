## Memory Safety
- A language is memory safe if it has ways to implement rules that prevent developers from making mistakes that can lead to memory-related bugs.
- Memory-related bugs are harder to diagnose and can be exploited as a vulnerability by hackers
- It ensures a program can correctly access & manage memory resources without causing unexpected behaviour or security issues

## Common Memory Issues

### Buffer Overflow
- This occurs when a program writes more data to a fixed-size buffer than it can hold.
- This can cause excess data to overwrite adjacent memory locations and lead to crashes or data corruption, etc

### Use-After-Free
- This happens when a program continues to use a pointer or ref to memory that has been freed or deallocated.
- Accessing freed memory can lead to crashes & unpredictable behaviour

### Data Races
- This happens when two or more threads access the same memory location concurrently and at least one of them performs a write opr

## Rust Addressing Memory Safety Issues
- For buffer overflow, Rust enforces strict bound checking on arrays and other collections, preventing program to write more data than it can hold.

- For Use-After-Free, Rust's ownership sys and borrwoing rules ensure once a mem is deallocated or moved, further access to it is prevented by the compiler.

- For Data Races, Rust's enforces that either a single mutable ref or multiple immutable ref can access the data at any given time which provies safe concurrent access to mem locations.


## Understanding Variable Lifetimes
 - Memory is allocated for a variable when you create it & it becomes valid
 - The variable can be used within its scope & Rust ensures no other part of code can access or modify the memory it occipies
 - Once a variable is out of scope, Rust automatically deallocates its mem
