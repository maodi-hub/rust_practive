// trait 类似于class的实现
trait Shape {
    fn area(&self) -> f64 {
        24.0
    }
}

trait Round {
    fn get_radius(&self) -> f64;
}

struct Circle {
    radius: f64
}

// 第一个入参不是self 即静态方法需要 类型名::方法名调用，其余类型方法通过点语法调用
impl Circle {
    // 静态方法
    fn haha() {
        println!("asdasd");
    }
}
// impl（可以实现方法扩展） 来表示实现class中的方法, for 指定扩展方法的对象
// impl Shape for Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * self.radius * self.radius
//     }
// }

impl Round for Circle {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Shape for dyn Round {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.get_radius() * self.get_radius()
    }
}



fn main() {
    println!("Hello, world!");

    // let c = Circle { radius: 64.0 };
    let c = Box::new(Circle { radius: 64.0 }) as Box<dyn Round>;
    println!(" the area is {}", c.area());
    Circle::haha();
}
