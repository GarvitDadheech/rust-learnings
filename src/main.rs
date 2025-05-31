fn main() {
    println!("Hello, world!");
    println!("{}", is_even(101));
    println!("{}", fibonnaci(10));
    println!("{}",get_str_length(String::from("Garvit Dadheech")))
}

fn is_even(n: i32) -> bool {
    if n % 2 ==0 {
        return true;
    }
    return false;
}

fn fibonnaci(n: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    if n == 0 {
        return 0;
    }
    if n ==1 {
        return 1;
    }
    for _ in 1..n {
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}

fn get_str_length(str: String) -> usize {
    str.chars().count()
}