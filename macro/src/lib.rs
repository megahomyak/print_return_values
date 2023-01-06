use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn print_return_values(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{}", attr.to_string());
    println!("{}", item.to_string());
    item
}
