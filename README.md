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