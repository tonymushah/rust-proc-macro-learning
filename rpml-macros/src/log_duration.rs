use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub(crate) fn log_duration_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;
    let statements = block.stmts;
    let function_identifier = sig.ident.clone();

    quote! {
        #(#attrs)*
        #vis #sig {
            let __start = std::time::Instant::now();
            let __result = {
                #(#statements)*
            };
            println!("{} took {}μs", stringify!(#function_identifier), __start.elapsed().as_micros());
            __result
        }
    }.into()
}
