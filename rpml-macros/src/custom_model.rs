use darling::util::PathList;
use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
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
    let CustomModel {
        name,
        fields: target_field,
        extra_derives,
    } = model;
    let mut new_fields = quote! {};
    for Field {
        attrs,
        vis,
        ident,
        colon_token,
        ty,
        ..
    } in fields
    {
        let Some(ident) = ident else {
            panic!("Failed to get struct field identifier");
        };

        let path = match Path::from_string(&(ident.clone().to_string())) {
            Ok(path) => path,
            Err(error) => panic!("Failed to convert field identifier to path: {error:?}"),
        };
        if !target_field.contains(&path) {
            continue;
        }
        new_fields.extend(quote! {
            #(#attrs)*
            #vis #ident #colon_token #ty,
        });
    }
    let struct_ident = match Ident::from_string(name) {
        Ok(ident) => ident,
        Err(error) => panic!("{error:?}"),
    };
    let mut extra_derives_output = quote!();
    if !extra_derives.is_empty() {
        extra_derives_output.extend(quote! {
            #(#extra_derives,)*
        })
    }
    quote! {
        #[derive(#extra_derives_output)]
        pub struct #struct_ident {
            #new_fields
        }
    }
}

pub(crate) fn custom_model_impl(item: TokenStream) -> TokenStream {
    let original_struct = parse_macro_input!(item as DeriveInput);
    let DeriveInput { data, .. } = original_struct.clone();
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
