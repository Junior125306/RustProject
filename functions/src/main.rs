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
