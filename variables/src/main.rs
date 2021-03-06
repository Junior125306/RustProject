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
