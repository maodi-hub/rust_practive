fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello");
    s.push_str(" world!!!");

    // {
    //     let s1 = s.clone();
    //     println!("inner {}", s1);
    // }
    takes_ownership(&s);
    println!("hello {}", s);

    let mut s1 = gives_ownership();
    println!("new {}", s1);

    let s1_len = caculate_lenth(&s1);
    println!("s1 len {}", s1_len);

    change(&mut s1);
    println!("append byte  {}", s1);

    println!("str slice1 {}", &s1[2..]);
    println!("str slice2 {}", &s1[..2]);
    println!("str slice3 {}", &s1[..]);
}

fn takes_ownership(s: &String) {
    println!("Received {}", s);
}

fn gives_ownership() -> String {
    String::from("hello")
} // 返回了String的所有权

fn caculate_lenth(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("new byte");
}