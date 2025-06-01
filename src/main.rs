fn main() {
    println!("Hello, world!");
    println!("{}", is_even(101));
    println!("{}", fibonnaci(10));
    println!("{}",get_str_length(String::from("Garvit Dadheech")));

    let user = User {
        name: String::from("Garvit"),
        age: 20,
        gender: String::from("Male")
    };

    println!("{}",user.name);

    let rect = Rect{
        height: 2,
        width:4
    };
    println!("{}", rect.area());
    println!("{}", rect.params(2));
    println!("{}", Rect::noparams()); // static

    let sq = Shape::Square(2.0);
    let rect = Shape::Rectange(3.2, 4.3);
    find_area(sq);
    find_area(rect);
}

fn find_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Rectange(a,b ) => a*b,
        Shape::Square(a) => a*a
    }; 
    println!("{}",area);
    return area;
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

struct User {
    name: String,
    age: u32,
    gender: String
}

struct Rect {
    width: u32,
    height: u32
}
// impl
impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height
    }

    fn params(&self,num: u32) -> u32 {
        return num;
    }

    fn noparams() -> u32 {
        return 1;
    }
}

enum Shape {
    Square(f64),
    Rectange(f64,f64),
}