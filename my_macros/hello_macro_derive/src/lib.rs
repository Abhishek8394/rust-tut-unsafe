/// Procedural macros need to be on their own separate crate!
/// This one defines the hello macro.
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream{
    let name = &ast.ident; // (Struct name as Ident structure) on which the derive macro was used.
    // create rust code
    let gen = quote! {
        impl HelloMacro for #name{
            fn hello(){
                println!("Hello, Macro! I am {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream{
    // convert rust tokens into syntax tree
    // Returns a `DeriveInput` struct.
    let ast = syn::parse(input).unwrap();
    // add the implementation of trait
    impl_hello_macro(&ast)
}