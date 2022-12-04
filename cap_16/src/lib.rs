// use std::error::Error;

use proc_macro::{TokenStream};
// use syn::DeriveInput;

#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
    
    item
}
