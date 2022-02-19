use rand::Rng;
use std::io;

fn main() {
    println!("猜数!");
    println!("猜测一个数字!");

    let secret_number = rand::thread_rng().gen::<i32>();
    println!("神秘数字是{}", secret_number);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜测得数字是{}", guess)
}
