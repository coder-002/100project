use std::io;

fn main() {
    println!("Simple Calculator");

    println!("Enter the first number");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read input");
    let first_number : f64 = match first_number.trim().parse() {
        Ok(num)=> num,
        Err(_)=> {
            println!("Enter the valid number");
            return;
        }
    };    
    
    println!("Available operations : +, -, *, /");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read operator");
    let operator = operator.trim();

    println!("Enter the second number");
    let mut second_number = String::new(); 
    io::stdin().read_line(&mut second_number).expect("Failed to read input");
    let second_number : f64 = match second_number.trim().parse() {
        Ok(num)=> num,
        Err(_)=> {
            println!("Enter the valid number");
            return;
        }
    };    

    let result = match operator{
        "+" => add(first_number, second_number),
        "-" => subtract(first_number, second_number),
        "*" => multiply(first_number, second_number),
        "/" => {
             if second_number ==0.0{
                println!("Error: Division by zero is not allowed");
                return;
            }

            divide(first_number, second_number)},

        _=> {
            println!("Invalid operator. Please use +, -, *,/");
            return;
        }
    };


    println!("{} {} {} = {}", first_number, operator, second_number, result);
    
}


fn add(first:f64, second:f64)->f64{
    first + second
}

fn subtract(first:f64, second:f64)->f64{
    first - second
}
fn multiply(first:f64, second:f64)->f64{
    first * second
}
fn divide(first:f64, second:f64)->f64{
       first / second
}
