#![allow(dead_code, unused_variables)]
fn main() {
    let mut str = String::from("hello world");
    let r1 = &str;
    let r2 = &str;
    println!("{}, {}", r1, r2,);
    let r3 = &mut str;
    println!("{}", r3);

    // === 可变引用
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);
    // let mut s2 = String::from("hello");
    // change_string(&mut s2);
    // println!("string: {}", s2);
    // let r1 = &mut s2;
    // // let r2 = &mut s2;
    // // print!("{}, {}", r1, r2);

    // === 引用
    // let x = 5;
    // let y = &x;
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // ===
    // let mut str = String::from("hello");
    // str.push_str(", world!");
    // // let s2 = str;
    // takes_ownership(str);
    // // println!("{}", str);

    // let x = 4;
    // makes_copy(x);
    // println!("{}", x)
}
fn change_string(str: &mut String) {
    str.push_str(", world");
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
