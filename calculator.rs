fn main() {
    println!("----------------------------");
    let x = 5;
    let y = 3;
    println!("I am giving {} and {} as inputs", x, y); // use placeholders to pass values dynamically
    let add = add(x, y);
    let sub = sub(x, y);
    let mul = mul(x, y);
    let div = div(x as f64, y as f64); // typecaste i32 to f64 while passing the arguments
    println!("The results of the arithmetic operations were:\nAdd:{}\nSub:{}\nMul:{}\nDiv:{}", add, sub, mul, div);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // return directly without using "return" keyword and a semicolon
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: f64, b: f64) -> f64 {
    a / b
}
