use std::sync::atomic::AtomicBool;

use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, ImplItemFn};
use quote::quote;

static STATEMENT_IS_MADE: AtomicBool = AtomicBool::new(false);

#[proc_macro_attribute]
pub fn make_statement(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ImplItemFn);
    if STATEMENT_IS_MADE.swap(true, std::sync::atomic::Ordering::Relaxed) {
        return syn::Error::new(input.span(), "Statement can only be made one time").to_compile_error().into();
    }

    let statement = syn::parse2::<syn::Stmt>(quote! {
        println!("Those who dare to fail miserably can achieve greatly.");
    }).unwrap();
    input.block.stmts.insert(0, statement);

    let output = quote! {
        #input
    };

    output.into()
}
