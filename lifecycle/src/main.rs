fn main() {
    println!("Hello, world!");
}

fn demo1() {
    {
        // let r;
        // {
        // let x = 5;
        // r = &x; // 这样会报错 因为出了花括号r就失效了  不能引用户
        // }
        // println!("r:{}", r)
    }
}
// 此时会报错 函数返回一个引用的值 但是不知道这个值来自x还是来自y
// 可以加泛型解决 表示xy与返回值生命周期是一样的
fn demo2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
