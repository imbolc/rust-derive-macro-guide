use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(my_trait), forward_attrs(allow, doc, cfg))]
struct Opts {
    answer: Option<i32>,
}

#[proc_macro_derive(MyTrait, attributes(my_trait))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let answer = match opts.answer {
        Some(x) => quote! {
            fn answer() -> i32 {
                #x
            }
        },
        None => quote! {},
    };

    let output = quote! {
        impl MyTrait for #ident {
            #answer
        }
    };
    output.into()
}
