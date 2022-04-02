pub mod generate{
    use proc_macro_basic::generate;

    generate!("proc_macro_basic/fixtures/person.json");
}

use generate::*;
fn main() {
    
}