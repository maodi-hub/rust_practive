use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");
    println!("Your raw input is: {:?}.", input); // 打印输入的原始内容;
    let num: i64 = input.trim().parse().expect("please input a number");

    println!("number is :{}", num);
}
