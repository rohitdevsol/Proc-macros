use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(Diff)]
pub fn diff(input: TokenStream) -> TokenStream {
    // we are using syn for the ast
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    // for printing the ast
    // eprintln!("AST: {:#?}", ast);

    // parsing the ast
    let pairs = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields_named) => {
                    fields_named
                        .named
                        .iter()
                        .map(|field| {
                            // need to get this
                            let field_name = &field.ident;
                            let field_type = &field.ty;

                            quote! {
                                if self.#field_name != other.#field_name {
                                    changes.push(format!(
                                            "{} changed from {:?} → {:?}",
                                            stringify!(#field_name),
                                            self.#field_name,
                                            other.#field_name
                                    ));
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                }
                _ => {
                    panic!("Diff can only be derive for the structs");
                }
            }
        }
        _ => {
            panic!("Diff can only be derive for the structs");
        }
    };

    let expanded = quote! {
        impl traits::Diff for #name {
            fn diff(&self, other: &Self) -> Vec<String> {
                let mut changes = Vec::new();

                #(#pairs)*

                changes
            }
        }
    };

    // recompiling the ast to rust itself

    expanded.into()
}

// what do we want ?

// what do we want ??
// let diff = a.diff(b);
// ["age changed form : {} to {}", "name changed form : {} to {}"]

// let a = Person {
//     name: String::from("subham"),
//     age: 21,
// };

// let b = Person {
//     name: String::from("subh"),
//     age: 22,
// };

// a.diff(b)
// ["age changed from 21 to 22", "name changed from subham to subh"]
