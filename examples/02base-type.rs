fn main() {
    // integer i32, float f64
    let x = 7;
    let _y = 3.0;
    // assert_eq!(0.1 + 0.2, 0.3);
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    println!("{}", abc.1.to_bits());

    println!("{}", plus_or_minux(x));
}

fn plus_or_minux(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5 // 返回表达式，不能加分号结尾
}
