use std::collections::HashMap;

fn main() {
    // 声明后赋值
    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // 声明并赋值
    // let mut v = vec![1, 2, 3];
    // let third: &i32 = &v[2];
    // println!("3333333  {}", third);

    // match v.get(100) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element"),
    // }
    // for item in &mut v {
    //     *item += 50;
    //     println!("item{}", item)
    // }
    // // 创建String
    // let data = "init text";
    // let mut s = data.to_string();
    // let s1 = "init text".to_string();
    // let s2 = String::from("init text");
    // // 更新String
    // s.push_str(data); // 字符串
    // println!("{}{}", s, data);

    // s.push('😊'); // 字符
    // println!("{}{}", s, data);

    // // 拼接字符串 第二个要求是引用
    // // 使用 + 号时 相当于使用了一个类似add方法 add(self,&str) 此时 s1 已经被remove
    // // let s3 = s1 + &s2;
    // //format! 宏不会remove
    // let s3 = format!("{}{}", s1, s2);
    // println!("{}{}", s3, s3.len());

    // // rust 看待字符串的三种方式 Bytes 字节, Scalar Values 标量值, grGrapheme Clusters 字形簇
    // // for b in s3.bytes() {
    // //     println!("{}", b)
    // // }
    // // for b in s3.chars() {
    // //     println!("{}", b)
    // // }
    // HashMap 数据存放在heap上
    // 创建后添加数据
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    // 所有的key都是一个类型  所有的value都是一个类型

    let teams = vec!["Blue".to_string(), "Red".to_string()];
    let init = vec![10, 40];
    let source: HashMap<_, _> = teams.iter().zip(init.iter()).collect();
    let aa = source.get(&String::from("Blue"));
    // HashMap 取值
    match aa {
        Some(s) => println!("{}", s),
        None => print!("11111111111"),
    }
    // 遍历hashmap
    for (k, v) in &source {
        println!("{},{}", k, v)
    }

    // 更新hashmap
    // 1.替换现有的
    scores.insert(String::from("Blue"), 25);
    // 2.保留现有的忽略新的
    // 需要检查是否有这个key
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // 3.合并现有的 和新的
    println!("{:?}", scores);
}
