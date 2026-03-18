use proc_macro::TokenStream;
use syn::{ Data, DeriveInput, Fields, parse };
use quote::quote;
#[proc_macro_derive(ToMap)]
pub fn to_map(input: TokenStream) -> TokenStream {
    // generate AST using syn
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;

    // printing the AST
    // eprintln!("{:#?}", ast);

    let map_insert_code = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields_named) => {
                    fields_named.named
                        .iter()
                        .map(|field| {
                            let field_name = &field.ident;
                            quote! {
                                map.insert(stringify!(#field_name).to_string(), self.#field_name.to_string());
                            }
                        })
                        .collect::<Vec<_>>()
                }
                _ => {
                    panic!("Diff can only be derive for the structs");
                }
            }
        }
        _ => panic!("Only works with struct"),
    };

    let expanded =
        quote! {
        impl ToMap for #name{
            fn to_map(self)->HashMap<String,String>{
                let mut map = HashMap::new();
                #(#map_insert_code)*
                map
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(FromMap)]
pub fn from_map(input: TokenStream) -> TokenStream {
    // generate AST using syn
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;

    let (field_code, field_names) = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let mut field_code = Vec::new();
                    let mut field_names = Vec::new();

                    for field in &fields.named {
                        let ty = &field.ty;
                        let key = field.ident.as_ref().unwrap().to_string();
                        let ident = field.ident.as_ref().unwrap();

                        if quote!(#ty).to_string() == "String" {
                            field_code.push(
                                quote! {
                                    let #ident:#ty = map.get(#key)
                                    .expect("missing field")
                                    .to_string();
                                }
                            );
                        } else {
                            field_code.push(
                                quote! {
                                let #ident: #ty = map.get(#key)
                                    .expect("missing field")
                                    .parse::<#ty>()
                                    .expect("parse error");
                            }
                            );
                        }
                        field_names.push(quote! { #ident });
                    }
                    (field_code, field_names)
                }
                _ => panic!("Only works with named fields"),
            }
        }
        _ => panic!("Only works with structs"),
    };

    let expanded =
        quote! {
        impl FromMap for #name{
            fn from_map(map: HashMap<String,String>)->Self{
                // we need key and value
                // construct a new struct
                // let key: ty = value 
                // access map after expansion
                #(#field_code)*
                #name {
                    // age : 10 -> just an example
                    #(#field_names),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
