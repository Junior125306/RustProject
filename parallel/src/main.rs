// use std::sync::mpsc;
use std::{
    sync::{Arc, Mutex},
    thread,
};
// use std::thread;
// use std::time::Duration;
fn main() {}

// fn mutex_text() {
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }
//     println!("Result :{}", *counter.lock().unwrap());
// }

// fn mutex_demo() {
//     let m = Mutex::new(5);
//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//     println!("m = {:?}", m)
// }
// fn Channel_test() {
//     let (tx, rx) = mpsc::channel();
//     let tx1 = mpsc::Sender::clone(&tx);
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("1:hi"),
//             String::from("2:from"),
//             String::from("3:the"),
//             String::from("4:thread"),
//         ];
//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_millis(200));
//         }
//     });
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_millis(200));
//         }
//     });

//     for received in rx {
//         println!("Got:{}", received)
//     }
// }

// fn thread_test() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     handle.join().unwrap();
// }
