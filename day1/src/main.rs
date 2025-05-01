/*1. Temperature Converter
Write a function that converts a temperature from Celsius to Fahrenheit.
Formula: F = C * 9/5 + 32
Input: f64 Celsius value
Output: Print Fahrenheit result
Add basic input validation using if (e.g., reject temperatures below -273.15). */


fn temp_convert ( c : f64 ) -> f64 {
    let mut f:f64 = 0.0;
    if c>=-273.15 {
        f = c * (9.0/5.0) + 32 as f64; /*Using 9 instead of 9.0 will give error since rust doesnt promote type */
    }
    return f;
}


/*
2. Even or Odd Counter
Write a function that takes a number n and prints whether each number from 1 to n is even or odd.
Use a for loop and if statements.
Function signature: fn even_odd_counter(n: u32)*/

fn even_odd_counter(n: u32) {
    for num in 1..n{
        if num%2==0 {
            println!("Even number {}", num)
        }else{
            println!("Odd number {}", num)
        }
    }
}

/*
3. Simple Calculator
Write a function calculate(a: i32, b: i32, op: char) that performs +, -, *, or / based on the operator.
Use match to handle the operator
Handle divide-by-zero cases using if */

fn calculate(a: i32, b: i32, op: char) {
   if op=='/' && b==0 || a==0{
    println!("Divide by zero attempted")
   } else {
    let res: i32 = match op {
        '+' =>  a+b,
        '-' => a-b,
        '*' => a*b,
        '/' => a/b,
        _ => 0 /* Default case should always be at end */
    };
    println!("Result for {} {} {} is {}",a,op,b,res);
   }
}

/*
4. Grade Checker
Write a function that accepts a u8 score (0–100) and prints the corresponding grade:
90-100: A
80-89: B
70-79: C
60-69: D
Below 60: F
Use if-else chains or match.
*/

fn grade_calculator(score:u8){
    match score {
        90..=100 => println!("A"),
        80..=89 => println!("B"),
        70..=79 => println!("C"),
        60..=69 => println!("D"),
        0..=59 => println!("F"),
        _ => println!("Invalid Score"),
    }
}

/*
5. Factorial Calculator
Write a function that computes the factorial of a number using a for loop.
Function signature: fn factorial(n: u64) -> u64 */

fn factorial(n: u64) -> u64 {
    let mut fac: u64 = 1;

    for num in 1..=n {
        fac = fac * num;
    };
    fac
}

/*
6. FizzBuzz
Classic! For numbers from 1 to 50:
Print "Fizz" if divisible by 3
Print "Buzz" if divisible by 5
Print "FizzBuzz" if divisible by both
Otherwise, print the number */

fn fizzbuzz(){
    for num in 1..=50 {
        if num%3==0 && num%5==0{
            println!{"FizzBuzz {}",num}
        } else if num%3==0 {
            println!{"Fizz {}",num}
        } else if num%5==0 {
            println!{"Buzz {}",num}
        } else {
            println!{"{}",num}
        }
    }
}

/*Main Function*/
fn main(){
    println!("{}",temp_convert(37.0));
    even_odd_counter(4);
    calculate(0, 5, '/');
    grade_calculator(65);
    println!("{}",factorial(10));
    fizzbuzz();
}
