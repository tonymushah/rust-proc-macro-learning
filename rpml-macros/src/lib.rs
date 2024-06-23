use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

#[proc_macro_derive(IntoStringHashMap)]
pub fn derive_into_hashmap(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_indentifier = &input.ident;
    match &input.data {
        Data::Struct(DataStruct{ fields, .. }) => {
            let mut implementation = quote! {
                let mut hash_map = std::collections::HashMap::<String, String>::new();
            };
            for field in fields {
                let identifier = field.ident.as_ref().unwrap();
                implementation.extend(quote! {
                    hash_map.insert(stringify!(#identifier).to_string(), String::from(value.#identifier));
                })
            }
            quote!{
                #[automatically_derived]
                impl From<#struct_indentifier> for std::collections::HashMap<String, String> {
                    fn from(value: #struct_indentifier) -> Self {
                        #implementation

                        hash_map
                    }
                }
            }
        },
        _ => unimplemented!()
    }.into()
}