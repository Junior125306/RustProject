fn main() {
    let email = String::from("acb@126.com");
    let username = String::from("Nikky");
    // 语法糖
    let user1 = User {
        email,
        username,
        active: true,
    };
    println!(
        "Hello, User,{},{},{}",
        user1.email, user1.username, user1.active
    );
    let _user2 = User {
        email: String::from("acb@126.com"),
        username: String::from("Nikky"),
        ..user1 // 基于user1创建user2  ..user1会自动补充缺少的字段
    };

    // tuple struct
    let _black = Color(0, 0, 0);

    // 计算长方形面积
    let w = 30;
    let l = 50;
    println!("{}", area(w, l));
    //计算面积 元组
    let rect = (30, 50);
    println!("{}", area_by_yuanzu(rect));
    //计算面积 struct
    let rng = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}", rng);
    //Rectangle { width: 30, height: 50 }
    println!("{:#?}", rng);
    // Rectangle {
    //     width: 30,
    //     height: 50,
    // }
    // 使用truct的方法
    println!("{}", rng.area_by_self());

    println!("{}", area_by_struct(&rng));

    // 判断是否能包含另一个长方形
    let rng1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{}", rng.can_hold(&rng1));
    // 调用函数的函数
    println!("{:?}", Rectangle::square(10));
}

struct User {
    username: String,
    email: String,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 每个struct 允许用友多个impl块
impl Rectangle {
    // 方法
    fn area_by_self(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
struct Color(i32, i32, i32);

// 计算长方形面积
fn area(width: u32, length: u32) -> u32 {
    width * length
}

// 计算长方形面积元组
fn area_by_yuanzu(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

// 计算长方形面积struct
fn area_by_struct(rng: &Rectangle) -> u32 {
    rng.width * rng.height
}
