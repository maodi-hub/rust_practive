fn main() {
    println!("Hello, world!");
    compute();
    compare(true, false, true);
    assign_expression();
    composite_assign();
    block_expression();
    if_else_expression();
}

// 基础计算
fn compute() {
    let x = 100;
    let y = 1;
    println!("{} {} {} {}", x + y, x - y, x * y, x / y);
}

// 参数比较
fn compare(a: bool, b: bool, c: bool) -> bool {
    // a == b == c (rust 中禁止连续比较)
    if a == b {
        c
    } else {
        b
    }
}

fn assign_expression() {
    // 可变变量声明时需要带上mut关键字
    let mut a = 10;
    println!("{}", a);
    a = 22;
    println!("{}", a);

    // 禁止连续赋值，否则最终参数为空tuple；
    // let c = (a = 3);
    // println!("{:?}", c);
}

fn composite_assign() {
    let x = 2;
    let mut y = 3;
    y += x;
    y *= x;
    println!("{}", y);
}

fn block_expression() {
    let x = {
        println!("block hello world!");
    };
    let y = {
        println!("block hello world y!");
        5 // 表示返回值，函数返回值不加分号或return出去
    };

    println!("{:?} {}", x, y);
}

fn if_else_expression() {
    let a = 10;
    let c: std::cmp::Ordering;
    if a < 10 {
       c = std::cmp::Ordering::Less
    } else if a == 10 {
       c = std::cmp::Ordering::Equal
    } else {
       c = std::cmp::Ordering::Greater
    }

    println!("{:?}", c);
}
