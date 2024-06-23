use darling::util::PathList;
use darling::{FromAttributes, FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data::Struct, DataStruct, DeriveInput, Field, Fields, Ident, Path};

#[derive(Debug, FromMeta, Clone)]
struct CustomModel {
    name: String,
    fields: PathList,
    #[darling(default)]
    extra_derives: PathList,
}

#[derive(Debug, FromDeriveInput, Clone)]
#[darling(attributes(custom_model), supports(struct_named))]
struct CustomModelArgs {
    #[darling(default, multiple, rename = "model")]
    pub models: Vec<CustomModel>,
}

fn generate_custom_model(fields: &Fields, model: &CustomModel) -> proc_macro2::TokenStream {
    todo!()
}

pub(crate) fn custom_model_impl(item: TokenStream) -> TokenStream {
    let original_struct = parse_macro_input!(item as DeriveInput);
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = original_struct.clone();
    if let Struct(data_struct) = data {
        let DataStruct { fields, .. } = data_struct;
        let args = match CustomModelArgs::from_derive_input(&original_struct) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };
        let CustomModelArgs { models } = args;
        let mut output = quote! {};
        if models.is_empty() {
            panic!("Please specify at least 1 model using the `model` attribute");
        }
        for model in models {
            let generated_model = generate_custom_model(&fields, &model);
            output.extend(quote!(#generated_model));
        }
        output.into()
    } else {
        panic!("DeriveCustomModel can only be used with named structs")
    }
}
