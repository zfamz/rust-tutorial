// struct Struct {
//     e: i32,
// }

fn main() {
    // shadowing
    let x = 5;
    println!("The start value of x: {}", x);
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // 解构式赋值
    // let (a, b, c, d, e);
    // (a, b) = (1, 2);
    // [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct { e, .. } = Struct { e: 5 };
    // assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    // 变量解构
    // let (a, mut b): (bool, bool) = (true, false);
    // println!("a = {:?}, b = {:?}", a, b);
    // b = true;
    // assert_eq!(a, b);

    // 未使用变量错误
    // let _x = 5;
    // let y = 10;

    // 可变, 不可变
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
}
