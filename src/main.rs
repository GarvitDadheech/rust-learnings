fn main() {
    println!("Hello, world!");
    println!("{}", is_even(101));
}

fn is_even(n: i32) -> bool {
    if n % 2 ==0 {
        return true;
    }
    return false;
}