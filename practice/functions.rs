fn square(num: i32) -> i32 {
    return num * num;
}

fn main() {
    let number = 4;
    let result = square(number);
    println!("The square of {} is {}", number, result);
}
