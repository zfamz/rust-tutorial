fn main() {
    // integer i32, float f64
    let _x = 3;
    let _y = 3.0;
    // assert_eq!(0.1 + 0.2, 0.3);
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    println!("{}", abc.1.to_bits())
}
