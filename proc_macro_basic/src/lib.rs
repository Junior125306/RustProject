use proc_macro::TokenStream;
mod json_schema;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello(){ println!(\"hello world\")}".parse().unwrap()
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    TokenStream::default()
}
