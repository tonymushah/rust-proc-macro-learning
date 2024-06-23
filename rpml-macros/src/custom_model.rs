use darling::util::PathList;
use darling::{FromAttributes, FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data::Struct, DataStruct, DeriveInput, Field, Fields, Ident, Path};

pub(crate) fn custom_model_impl(item: TokenStream) -> TokenStream {
    let original_struct = parse_macro_input!(item as DeriveInput);
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = original_struct.clone();
    todo!()
}
