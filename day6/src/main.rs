//Option: Used to indicate if a value is present or none since Rust doesnt have a undefined/null type in rust

//Standard Declaration:
// pub enum Option<T> {
//     Some(T),
//     None,
// }

//Option<T> has two variants:
//Some(T) => To denote presence of value of type T and the value itself
//None => To denote absence of value like null values
fn calculate_sqroot(num: f32) -> Option<f32> {
    if num > 0.0 {
        Some(num.sqrt())
    } else {
        None
    }
}

fn option() {
    let result = calculate_sqroot(-5 as f32);
    match result {
        Some(value) => println!("Sqroot is {value}"),
        None => println!(" Negative integer found"),
    }
}

//Result: Used to handling errors and returning meaningful info about the cause of error.

// //Standard Declaration:
// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

//Result<T,E> has two variants:
//Ok(T) => To denote the success case of type T
//Err(E) => To denote the error case of type E
fn withdraw_bal(amt: f32) -> Result<f32, String> {
    let currbal = 120.0;
    if amt > currbal {
        Err("Withdraw amount more than current bal".to_string())
    } else {
        Ok(currbal - amt)
    }
}

fn result() {
    let withdraw_amt = 210.0;
    let trc = withdraw_bal(withdraw_amt);
    match trc {
        Ok(value) => println!("Remaining amount is {value}"),
        Err(msg) => println!("Error: {msg}"),
    }
}

fn main() {
    option();
    result();
}
