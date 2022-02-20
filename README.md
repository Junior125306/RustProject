# RustProject
Rust学习项目

开发工具：vscode
添加插件：rust-analyzer,Toml Language Support,crates

## Day1
安装： 官网下载
通过vsstudio 安装c++环境

rust程序后缀名 .rs

文件命名规范： hello_world.rs

- 更新rust `rustup update`
- 卸载rust `rustup self uninstall`
- 安装验证  `rustc --version`
- 查看本地文档 `rustup doc`
- 编译 `rustc main.rs`
- 查看cargo版本 `cargo --version`
- cargo创建项目 `cargo new projectname`
- cargo编译项目 `cargo build`
  - 创建可执行文件 `target\debug\hello_cargo.exe`
- 编译代码 + 执行结果 `cargo run`
  - 如果上次编译后代码没有被修改过 不会再次编译
- 检查代码 `cargo check` 确保能够通过编译,比`cargo build`快得多
- 发布 `cargo build --release` 编译时会进行优化 会在`target/release`下生成可执行文件

cargo.toml
- pacakge 是一个区域标题 表示配置当前包的
  - name 项目名
  - version 项目版本
  - authors 项目作者
  - edition 使用的rust版本
- dependencies 另一个区域的开始 配置依赖项的地方

新学习到的代码
```rust
    println!("打印一行字")
    // 读取一行字  mut 标识变量 不加mut表示常量不可被修改
    use std::io;
    let mut guess = String::new();
    // expect 类似catch  read_line会返回一个result 如果是err 则运行中断程序并循行 expect
    io::stdin()::read_line(&mut guess).expect("无法读取行");

    // 获取随机数
    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("神秘数字是{}", secret_number);

    // rust 允许重名变量  后面赋值的会把前面的覆盖掉  parse() 会把字符串解析成数字
    let guess: u32 = guess.trim().parse().expect("Please type a number");
    // try catch 写法
    let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
    };
```

## Day2 变量-标量-复合类型

路径 ./variables/

### 代码演示
```rust
const MAX_POINTS: u32 = 100_000;
fn main() {
    // 变量与可变性
    // var()
    // 标量类型
    // biao_linag()
    // 复合类型
    fu_he();
}

fn fu_he() {
    // rust 有两种复合类型 一种是元组(Tuple) 一种是数组
    // Tuple 可以将多个类型的多个值放到一个类型里 长度是固定的一旦被声明将无法更改
    // Tuple 中每个位置对应一个类型  各元素类型不必相同
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    print!("{},{},{}", tup.0, tup.1, tup.2);
    // 解构
    let (x, y, z) = tup;
    println!("{},{},{}", x, y, z);
    // 数组 可以将多个值放到一个类型里，且每个元素的类型必须相同  数组的长度也是固定的
    let a = [1, 2, 3, 4, 5];
    // 如果想让数组存到stack（栈） 上而不是heap（堆）上，或者想保证固定数量的元素，这时使用数组更有好处
    // 数组没有Vector灵活 Vector 长度可以改变
    // 如果数组中每个元素都相等可以下面这样声明
    let b = [5; 5];
    println!("{},{},{},{},{}", b[0], b[1], b[2], b[3], b[4])
    // 数组时Stack上分配的单个块的内存
    // 如果访问的索引超出了数组的范围 编译会通过 运行时会报错
    // rust不会允许其继续访问相应的地址内存
}

fn biao_linag() {
    // Rust 是静态编译语言，在编译时必须知道所有变量得类型
    //基于使用的的值编译器通常能推断出具体类型
    //但是如果可能的类型比较多 比如 String 转换为整数的parse方法就必须添加类型标注否则会报错
    // 例子  如果不标注u32 parse不知道自己要解析成什么 就会报错
    let guess: u32 = "42".parse().expect("not number");
    println!("{}", guess)
    // 整数类型 如 u32 是一个无符号（unsigned）整型 占据32位空间
    // 有符号范围 -(2ⁿ⁻¹) 到 2ⁿ⁻¹-1
    // isize 和 usize由计算机架构决定
    // 详情参考附表1
}

fn var() {
    println!("Hello, world!");
    // mut 修饰不可变变量  如果不加mut则是 不可变变量
    let mut x = 5;
    x = 6;
    println!("this value is {}", &mut x);
    // const 修饰常量 他的类型必须被标注清除
    // 常量可以在任何作用域内进行声明,包括全局作用域
    // 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值
    // 在常量运行期间 常量在其声明的作用域内一直有效
    // 常量命名规范：Rust 里的常量使用全大写字母,每个单词之间用下划线分开 例如： MAX_POINTS
    println!("this value is {}", MAX_POINTS);
    // Shadowing 隐藏
    // rust 允许使用相同的名字声明新的变量，新的变量就会shadow（隐藏）之前声明的同名变量
    let y = 5;
    // y = y + 1;  如果这样写的话会报错 因为是不可变的变量
    let y = y + 1; // 但是如果加上一个let 则把之前的y shadow掉了
                   // 在后续的代码中这个变量明代表的就是新的变量
                   //shadow 和 标记为 mut 是不一样的
                   //如果不使用let 那么重新给非mut的标量赋值会编译错误
                   //而使用let声明的新变量依旧是不可变的 并且新变量可以跟原来的变量类型不同
    println!("this y is {}", y);
}

```
### 整数

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

### 整数字面值

| Number        | Example     |
| ------------- | ----------- |
| Decimal       | 98_222      |
| Hex           | 0xff        |
| Octal         | 0o77        |
| Binary        | 0b1111_0000 |
| Byte(u8 only) | b'A'        |

### 整数溢出
例如：u8范围是0-255,如果把u8一个变量设置为256那么调试模式会报错移除 运行程序时会panic,发布模式下不会检查可能导致panic的整数溢出,如果溢出发生会通过环绕解决.

### 浮点型
Rust有两种浮点型，也就是含有小数部分类型 `f32 32位 单精度` `f64 64位 双精度`
Rust的浮点型用IEEE-754标准来表述
f64是默认类型，以为现在cpu f64 f32速度差不多

### 数值操作

和其他语言差不多

### 布尔类型
用 bool 描述  一个字节大小  值为 false  true

### 字符类型
Rust 使用 char类型来描述单个字符  使用单引号  占用四个字节大小
是Unicode标量值,可以标识比ASCLL 多得多的字符内容：拼音  中日韩文  零长度空白符 emoje表情等

## Day3 函数 控制流 循环
./functions/
```rust
fn main() {
    println!("Hello, world!");
    other_fun(21);
    let y = {
        let x = 1;
        x + 3
    };
    // 此时花括号内的代码是一个表达式 可以赋值给变量
    // 如果 x+3 没有分号 则会返回x+3的结果 如果有分号则返回一个()
    println!("{}", y);
    // 具有返回值的函数
    println!("{}", five(6));
    //if_else
    if_else(6);
    // 此时要求 6 7 类型一样 否则会报错
    let number = if true { 6 } else { 7 };
    println!("The number is {}", number);
    // rust 中有三种循环 loop，for,while
    // loop 循环 使用break;退出循环 continue;跳过本次循环
    loop_test();
    // while 判断条件如果不满足则执行花括号中的代码
    while_test();
    // for 循环
    for_test();
}

// rust 形参要注明类型
fn other_fun(x: i32) {
    print!("{}", x)
}

// 具有返回值的函数
// 通常返回值就是函数体里面最后一个表达式的值
// 若要提前返回则需要使用return关键字 并指定一个值
// 大多数函数都是使用最后一个表达式最为返回值
fn five(x: i32) -> i32 {
    x + 5
}

// if表达式 根据不同的条件执行不同的分支
fn if_else(number: i32) {
    if number < 5 {
        //如果不是bool类型则会报错 不会转换
        println!("number was bad")
    } else if number == 6 {
        println!("66666666666666")
    } else {
        println!("number was ok")
    }
}

fn loop_test() {
    let mut var1 = 0;
    let result = loop {
        var1 += 1;
        if var1 == 10 {
            break var1 * 2;
        }
    };
    println!("result = {}", result);
}

fn while_test() {
    let mut var2 = 3;
    while var2 != 0 {
        println!("var2 is {}", var2);
        var2 -= 1;
    }
    println!("while_test end")
}

fn for_test() {
    let a = [1, 2, 3, 4, 5];
    for item in a {
        println!("item is {}", item)
    }
}

```