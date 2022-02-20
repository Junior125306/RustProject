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

## Day4 所有权规则,内存预分配,所有权与函数,引用与借用,切片

Rust 的核心特性就是所有权
+ 所有程序在运行时都必须管理他们使用计算机内存的方式
  - 有些语言有垃圾收集机制，在程序运行时，他们会不断地寻找不再使用的内存
  - 在其他语言中，程序员必须显式地分配和释放内存

+ Rust的做法
  - 内存是通过一个所有权系统管理的，其中包含一组编译器在编译时检查的规则
  - 当程序运行时，所有权特性不会减慢程序的运行速度。

>对于Rust来说一个值存在Stack还是Heap上对语言的行为和你要做什么决定有很大的影响 Stack和Heap 存储数据的格式不同 

- Stack后进先出 
  - 添加数据叫做压入栈 
  - 移除数据叫做弹出栈
- 所有存储在stack上的数据必须拥有一致的固定的大小
  -编译时大小位置的数据或运行时大小可能发生变化的数据必须存放在heap上
- heap内存组织性差一些
  - 当你把数据放入heap时，会请求一定数量的控件
  - 操作系统在heap上找到一块足够大的空间，把他标记为再用，并返回一个指针(内存地址)

把值压到stack上不叫分配，因为指针是已知固定大小的，可以把指针存放到stack
把数据存放到stack比heap要快 因为不需要寻找新的数据空间，新的数据永远存放在顶端
heap中访问要慢一点，因为需要通过指针才能找到数据，对于现在浏览器来说，由于缓存的缘故，如果指令在内存中跳转次数越少，那么速度就越快。

### 所有权存在的原因：管理heap上的数据

### 所有权解决的问题
  - 跟踪代码的哪些部分在使用heap的哪些数据
  - 最小化heap上的重复数据量
  - 清理heap上未使用的数据避免空间不足

### 所有权规则

- 每个值都有一个变量，这个变量是该值的所有者
- 每个值同时只能有一个所有者
- 当所有者超出作用域（scope）时，该值将被删除。


### Stack 上的数据： 复制
- Copy trait，可以用于想整数这样完全存放在stack上面的类型
- 如果一个类型实现了copy这个trait 那么旧的变量在赋值后仍然可以使用
- 如果一个类型或者该类型的一部分时间了Drop trait，那么rust不允许让他再去时间Copy tarit
- 任何简单的标量组合类型都是可以Copy的
- 任何需要分配内存或某种资源的都不是Copy的

一些拥有Copy trait的类型
- 所有整数类型，例如 u32
- bool
- cahr
- 所有浮点类型 如 f64
- Tuple如果其中所有字段都是Copy的 那么他就是

> 一个变量的所有权总是遵循同样的模式

- 把一个值赋给其他变量时就会发生移动
- 当一个包含heap数据的变量离开作用域是，他的值就会被drop函数清理，除非数据的所有权移动到另一个变量上了

```rust
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
}
//rust 采用了不同的方式：对于某个值来说，当拥有它的变量走出作用范围时，内存会立即自动交还给操作系统.
//drop

fn show_Strng(try_string: String) {
    println!("{}", try_string)
}

fn give_me_string() -> String {
    let some_string = String::from("_");
    some_string // 此时 some_string 被move到调用这个函数的函数中
}

fn takes_and_back(some_string: String) -> String {
    some_string
}
```
![String图1](./img/String1.png)
![String图2](./img/String2.png)
![String图3](./img/String3.png)
### 引用和借用

>引用的参数是 &Sting 而不是 String
>&符号表示引用：允许你引用某些值而不取得其所有权

以引用作为参数的行为叫做借用
![引用图示](./img/String4.png)

不可变引用
```rust
fn main() {
      let s3 = String::from("_");
    show_Strngs(&s3);
    println!("s3他还在 {}", s3);
}

fn show_Strngs(try_string: &String) {
    println!("{}", try_string)
}
```

可变引用
```rust
fn main() {
      let mut s3 = String::from("_");
    show_Strngs(&mut s3);
    println!("s3他还在 {}", s3);
}

fn show_Strngs(try_string: &mut String) {
    try_string.pust_str("111");
    println!("{}", try_string)
}

在特定作用域内对某一块的数据只能有一个可变的引用 ， 这样做的好处是可以在编译时防止数据竞争
以下三种行为会发生数据竞争
- 两个或多个指针同时访问同一数据
- 至少有一个指针用于写入数据
- 没有使用任何机制来同步对数据的访问

可以通过创建新的作用域，来允许非同时的创建多个可变应用
如：

```rust
    fn main(){
        let mut s = String ::from("hello");
        {// 这是一个作用域
            let s1 = &mut s;
        }
        // 此时s1 已经被drop
        let s2  = &mut s;
    }
```

rust 不允许一个变量用友一个可变引用和一个不变引用

多个不变引用是可以的

#### 悬空引用 Dangling References

cpp中 指一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给其他人使用了

在rust中，编译器保证引用永远都不是悬空引用
    - 如果你引用了某些数据，编译器将保证在引用离开作用域之前数据不会离开作用域。

引用的规则 
- 在任何给定的时刻，只能满足下列条件之一
  - 一个可变的引用
  - 任意数量的不可变引用
- 引用必须一直有效

### 切片
    Rust 的另外一种不持有所有权的数据类型 切片（slice）
``` rust
    let mut s = String::from("hello World");
    println!("看看是啥把{}", first_word(&s));
    println!("看看是啥把字符串切片把{},{}", &s[0..5], &s[6..11]);
    // 语法糖  如果开头是0 可以不写 如果结尾是字符串长度可以不写
    println!("看看是啥把字符串切片把{},{}", &s[..5], &s[6..]);
    // 如果取全部都可以不写
    println!("看看是啥把字符串切片把{}", &s[..])

    let a = [1,22,111,222]
    let slice = &a[..]
    // 与字符串使用相同
```