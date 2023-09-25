enum Color {
    Red,
    Green
}

struct Price(u32);

enum FruitBox {
    Apple(Color),
    Pear(Color, Price)
}

fn main() {
    println!("Hello, world!");
    let apple = FruitBox::Apple(Color::Green);
    let pear = FruitBox::Pear(Color::Red, Price(64));

    eat_or_sell(&apple);
    eat_or_sell(&apple);
    eat_or_sell(&pear);

    if_eat_or_sell(&apple);
    if_eat_or_sell(&pear);

    do_a_lot_thing_if_is_apple(apple);
    do_a_lot_thing_if_is_apple(pear);
}

fn eat_or_sell(fruit: &FruitBox){
    match fruit {
        FruitBox::Apple(_) => eat_apple(),
        FruitBox::Pear(_, _) => sell_pear(),
    }
}

fn if_eat_or_sell(fruit: &FruitBox) {
    if let FruitBox::Apple(_) = fruit {
        eat_apple();
    } else {
        sell_pear();
    }
}

fn do_a_lot_thing_if_is_apple(fruit_box: FruitBox) {
    let FruitBox::Apple(_) = fruit_box else {
        sell_pear();
        return;
    };
    eat_apple();
    
}

fn eat_apple() {
    println!("吃苹果")
}

fn  sell_pear() {
    println!("出售梨")
}
