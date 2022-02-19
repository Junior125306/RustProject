use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("神秘数字是{}", secret_number);
    loop {
        println!("猜测一个数字!");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        // shadow  去掉空格 解析成数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测得数字是{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Equal => {
                println!("你赢了");
                break;
            }
            Ordering::Greater => println!("太大了"),
        }
    }
}
