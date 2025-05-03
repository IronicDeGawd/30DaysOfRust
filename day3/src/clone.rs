/* Clone Function: Copies the reference from stack and the actual value from heap of a variable as well. It basically creates a deep copy of a value */
fn clone() {
    let mut org_string = String::from("Sounds Great!");
    let cloned_string = org_string.clone();

    org_string.push_str(" Doesn't it?");
    println!("Modified Org String: {}", org_string);
    println!("Cloned String: {}", cloned_string);

    /* Calling the modify function to create a modified string using immutable reference */
    let org_str = String::from("String Created");
    let modified_str = modify(&org_str);

    println!("{}", org_str);
    println!("{}", modified_str);
}

/* Clone Function can also be used to modify a string using a immutable reference by cloning it and modifying it. */
fn modify(str: &String) -> String {
    let mut modified_str = str.clone();
    modified_str.push_str(" & Modified");
    modified_str
}

fn main() {
    clone();
}
