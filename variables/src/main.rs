fn main() {
    let mut x = 5;
    const CONSTANT: usize = 100;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("{}", y);
    }

    println!("{}", y);

    let some_string = "aaa";
    println!("{:p}", &some_string);

    let some_string = some_string.len();
    println!("{:p}", &some_string);
}
