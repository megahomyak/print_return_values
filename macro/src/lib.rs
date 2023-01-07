use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn print_return_values(attr: TokenStream, item: TokenStream) -> TokenStream {
    assert!(
        attr.into_iter().next().is_none(),
        "there should be no arguments passed to this macro"
    );
    let item = parse_macro_input!(item);
    let syn::Item::Fn(function) = item else {
        panic!("this macro can only be applied to a function")
    };
    let body_block = function.block;
    let signature = function.sig;
    let visibility = function.vis;
    let function_name = &signature.ident;
    // Attributes (`function.attrs`) are **NOT** covered because donk them
    let result = quote!(
        #visibility #signature {
            let return_value = (move || #body_block)();
            println!(concat!(stringify!(#function_name), " -> {:?}"), return_value);
            return_value
        }
    );
    result.into()
}
