use std::collections::HashMap;

//Collections : They are powerful and versatile tools that enable you to store multiple data elements in a single structure.

fn main() {
    vector();
    string();
    hashmap();
}

//Vectors : Dynamic array that can shrink or grow and store same type of datatype. It is stored on heap for dynamic allocation.
fn vector() {
    println!("\t \n Vectors: ");

    //Two ways to create vectors:
    let mut _number = vec![1, 2, 3, 4];
    let mut name: Vec<String> = Vec::new();

    name.push("Aditya".to_string());
    name.push("Srivastava".to_string());

    //& is used to pass reference of the value from a vector because rust doesnt allow moving ownership of a value in a vector
    let _fname: &str = &name[0];

    println!("F_Name: {}, L_Name: {}", &name[0], &name[1]);
    name.pop(); //Last index of vector is popped
    println!("{:?}", name);

    name.push("Srivastava".to_string());
    name.push("Aarush".to_string());
    name.push("Srivastava".to_string());

    let name_slice = &name[1..4]; //Index 2, 3, 4
    println!("{:?}", name_slice);
}

//Strings :  They are a collection of characters, implemented over vectors.
fn string() {
    //String::from() : Used to implement growable, mutable & heap-allocated data
    //&str : Used to implement immutable, borrowed ref to a string slice or a part of string

    println!("\t \n Strings: ");

    //Two ways to create string:
    let mut mystring = String::from("Hello");
    let mut _sec_mystring = " World!".to_string();

    //String slice
    let _str_slice: &str = &mystring[2..5];

    //push_str is used to push in the end of current string
    mystring.push_str(&_sec_mystring);

    println!("{mystring}");

    //string.chars is used to access the chars present in a string
    println!("{:?}", mystring.chars());
    for c in mystring.chars() {
        println!("{c}")
    }
}

//Hashmaps : They are type of dictionary, storing in a key-value pair
fn hashmap() {
    println!("\t \n Hashmaps: ");
    //HashMaps use Options<> to store values with Some for value and None if it doesnt exist
    let mut scores = HashMap::new();

    //hashmap.insert(key, value) : insert values
    //hashmap.get(key) : get value of a key
    //hashmap.remove(key) : remove keypair from hashmap

    scores.insert(String::from("Aditya"), 20); //[Aditya: 20]
    scores.insert(String::from("Aarush"), 16); //[Aarush: 16]

    let adi_score = scores.get(&String::from("Aditya"));

    for (k, v) in &scores {
        println!("{k}: {v}")
    }

    println!("{:?}", scores);
    println!("{:?}", adi_score); //prints Some(20)

    scores.remove(&String::from("Aarush"));
    println!("{:?}", scores);

    for (k, v) in &scores {
        println!("{k}: {v}")
    }
}
