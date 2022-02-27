use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("解析参数获取错误{}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Application error :{}", e);
        process::exit(1);
    }
}
