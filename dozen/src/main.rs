use std::{thread, time::Duration};

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("got:{}", val)
    }
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number)
}

fn generate_workout(instensity: u32, random_number: u32) {
    // 声明一个闭包
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculation slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if instensity < 25 {
        print!("Today, do {} pushups!", expensive_closure.value(instensity));
        println!("Next,do {} situps!", expensive_closure.value(instensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remeber to stay hydrated!")
        } else {
            println!(
                "Today,run for {} minutes!",
                expensive_closure.value(instensity)
            )
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                v
            }
        }
    }
}
