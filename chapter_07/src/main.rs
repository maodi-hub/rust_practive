mod apple;
mod pear;
mod orange;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let r = rand::thread_rng().gen::<i64>() % 2;
    if  r == 0 {
        println!("hello");
        apple::eat_apple();
    } else {
        println!("world");
        pear::eat_pear();
    }

    orange::eat::eat_orange();
}
