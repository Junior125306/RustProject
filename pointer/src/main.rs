use std::{cell::RefCell, ops::Deref};

struct TestDrop {
    data: String,
}

impl Drop for TestDrop {
    fn drop(&mut self) {
        println!("droping with data {}!", self.data)
    }
}
fn main() {
    let m = MyBox::new(String::from("rust"));
    // 实现了deref trait后 可以这样写
    // 因为 deref 会把 &MyBox<String>引用 转换为 &String 引用
    // 然后因为String实现了deref 从而 &String引用可以转换为&str引用
    hello(&m);
    // 如果没有实现的话只能怎么写
    // hello(&[*m][..]);
    // my_box()
    my_deref();

    let c = TestDrop {
        data: String::from("_"),
    };
    println!("==============c{}", c.data)
}

// fn my_box() {
//     let b = Box::new(5);
//     println!("b==={}", b)
// }
fn hello(name: &str) {
    println!("hello,{}", name)
}

fn my_deref() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let m = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *m);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
#[test]
fn rc_test() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let b = Cons(4, Rc::clone(&a));
}
