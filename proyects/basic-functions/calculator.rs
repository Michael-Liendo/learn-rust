fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn subtraction(x: i32, y: i32) -> i32 {
    x - y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn division(x: i32, y: i32) -> i32 {
    x / y
}

fn main() {
    let result: i32 = sum(2, 3);
    println!("{}", result);
}
