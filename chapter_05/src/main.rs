struct Point {
    x: f64,
    y: f64,
}

struct Color(u8, u8, u8);

struct SingleStruct;

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle { width: w, height: h }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Person {
    name: String
}
impl Person {
    fn greet(&self) {
        println!("person name {}", self.name)
    }

    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    // Consuming method
    fn get_name_and_consume(self) -> String {
        self.name
    }
}

fn main() {
    println!("Hello, world!");

    let p = create_point(3.0, 4.0);

    println!("point {} {}", p.x, p.y);

    let c = Color(255,255,255);
    let _s = SingleStruct;

    println!("point {} {} {}", c.0, c.1, c.2);

    let rectangle = Rectangle {
        width: 50,
        height: 60
    };

    println!("area {}", rectangle.area());

    let mut person = Person {
        name: String::from("xiaowang")
    };

    person.greet();

    person.change_name(String::from("lisi"));

    person.greet();

    let name = person.get_name_and_consume();

    println!("person current name {}", name);

    let new_rectangle = Rectangle::new(30, 500);

    println!("new rectangle {}", new_rectangle.area());
}

fn create_point(x: f64, y: f64) -> Point {
    Point { x, y }
}