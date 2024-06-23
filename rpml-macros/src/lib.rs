use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

#[proc_macro_derive(IntoStringHashMap)]
pub fn derive_into_hashmap(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_indentifier = &input.ident;
    match &input.data {
        Data::Struct(DataStruct{ fields, .. }) => {
            let field_identifiers = fields.iter().map(|field| field.ident.as_ref().unwrap()).collect::<Vec<_>>();
    
            quote!{
                #[automatically_derived]
                impl From<#struct_indentifier> for std::collections::HashMap<String, String> {
                    fn from(value: #struct_indentifier) -> Self {
                        let mut hash_map = std::collections::HashMap::<String, String>::new();

                        #(
                            hash_map.insert(stringify!(#field_identifiers).to_string(), String::from(value.#field_identifiers));
                        )*

                        hash_map
                    }
                }
            }
        },
        _ => unimplemented!()
    }.into()
}

#[proc_macro_derive(DeriveCustomModel, attributes(custom_model))]
pub fn derive_custom_model(item: TokenStream) -> TokenStream {
    todo!()
}