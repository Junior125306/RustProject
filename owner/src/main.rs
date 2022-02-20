fn main() {
    let mut s = String::from("hello");
    s.push_str(",World");
    println!("s is {} ", s);
    // 变量和数据交互的方式：移动（Move）
    // 多个变量可以与同一个数据使用一种独特的方式来交互
    let x = 5;
    let y = x;
    // 整数时已知且固定大小的简单的值，这两个5被压到了stack中

    let mut q = String::from("hello");
    let p = q;
    // String 与整数不同 一个String有三个部分组成 (参考String图1)
    // 1.一个指向内存中存放字符串内容的指针 ptr
    // 2.一个长度 len
    // 3.一个容量 capacity
    // 上面三个存放在stack 存放字符串内容的部分在heap 长度len就是存放字符串内容所需的字节数
    // 容量capacity是指String从操作系统总共获得内存的总字节数
    // 当q赋值给p时  String的数据被赋值了一份 : 在Stack上赋值了一份指针长度容量 并没有赋值heap上的数据 (参考String图2)
    // 当变量离开作用域时,Rust会自动调用drop函数,并将变量使用的heap内存释放
    // 当p q都离开作用域时他们会都释放相同的内存  二次释放（double free） bug
    // 为了保证内存的安全Rust没有尝试复制被分配的内存  会让q失效 当q离开作用域时 rust不会释放任何的东西
    // 如果真的是想要深拷贝可以使用clone方法 如 let p = q.clone(); 参考String图3

    // 所有权与函数
    let some_string = String::from("_");
    show_Strng(some_string); // 此时 some_string 被move到函数中的tryString上面
                             // print!("some_string {}", some_string);  // 此时会报错提示 some_string 已经被移动  但是如果不是sTring类型 是整数类型的话将不会报错

    let s1 = give_me_string();
    let s2 = String::from("_");
    let s3 = takes_and_back(s2);

    show_Strngs(&s3);
    println!("s3他还在 {}", &s3);
    let mut s = String::from("hello World");
    println!("看看是啥把{}", first_word(&s));
    println!("看看是啥把字符串切片把{},{}", &s[0..5], &s[6..11]);
    // 语法糖  如果开头是0 可以不写 如果结尾是字符串长度可以不写
    println!("看看是啥把字符串切片把{},{}", &s[..5], &s[6..]);
    // 如果取全部都可以不写
    println!("看看是啥把字符串切片把{}", &s[..]);
}
//rust 采用了不同的方式：对于某个值来说，当拥有它的变量走出作用范围时，内存会立即自动交还给操作系统.
//drop

fn show_Strng(try_string: String) {
    println!("{}", try_string)
}

fn show_Strngs(try_string: &String) {
    println!("{}", try_string)
}

fn give_me_string() -> String {
    let some_string = String::from("_");
    some_string // 此时 some_string 被move到调用这个函数的函数中
}

fn takes_and_back(some_string: String) -> String {
    some_string
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
