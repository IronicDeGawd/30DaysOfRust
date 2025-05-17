fn main() {
    println!("Hello, user!");
    let mut oper: String = String::new();
    println!("Enter preferred operation {{+, -, *, /}}: ");
    std::io::stdin().read_line(&mut oper).unwrap();
    let oper = oper.trim();

    let mut input = String::new();

    println!("Enter first number:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n1: f64 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter second number:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n2: f64 = input.trim().parse().expect("Invalid input");

    let operation: Operation = match oper {
        "+" => Operation::Add(n1, n2),
        "-" => Operation::Subtract(n1, n2),
        "*" => Operation::Multiply(n1, n2),
        "/" => Operation::Divide(n1, n2),
        _ => {
            println!("Invalid operation: {}", oper);
            return;
        }
    };

    let res: f64 = calculate(operation);
    println!("Result of {n1} {oper} {n2} = {res}");
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}
