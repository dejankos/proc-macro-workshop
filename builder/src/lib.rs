extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};
use syn::export::quote::__private::Ident;
use syn::spanned::Spanned;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", &ast);

    let struct_name = &ast.ident;
    let b_name = builder_name(struct_name);
    let b_ident = Ident::new(&b_name, ast.span());


    let exp = quote! {

        // builder struct
        struct #b_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        // custom struct impl builder
        impl #struct_name {
            fn builder() -> #b_ident {
                #b_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir:None,
                }
            }
        }
    };

    exp.into()
}

fn builder_name(struct_name: &Ident) -> String {
    format!("{}Builder", struct_name)
}
