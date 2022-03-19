#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("over 90%")
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("over 75")
        }
    }
}

mod tests {
    use super::*;
    struct Mock {
        sent: RefCell<Vec<String>>,
    }
    impl Mock {
        fn new() -> Mock {
            Mock {
                sent: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for Mock {
        fn send(&self, message: &str) {
            self.sent.borrow_mut().push(String::from(message))
        }
    }

    #[test]
    fn test() {
        let mock = Mock::new();
        let mut limit_tracker = LimitTracker::new(&mock, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock.sent.borrow().len(), 1);
    }
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut() += 10;
}
