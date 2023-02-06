extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn, ReturnType, FnArg};

#[proc_macro_attribute]
pub fn time_it(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let attrs = &input.attrs;
    let visibility = &input.vis;
    let signature = &input.sig;
    let block = &input.block;

    let fn_arg_types = &signature.inputs
        .iter()
        .map(|v| match v {
            FnArg::Receiver(rec) => {
                let mut res = String::with_capacity(9);

                if rec.reference.is_some() {
                    res.insert(0, '&');
                }
                if rec.mutability.is_some() {
                    res.insert_str(res.len(), "mut");
                }

                let s = if res.len() > 1 { " self" } else { "self" };
                res.push_str(s);
                res
            },
            FnArg::Typed(pat_type) => format!("{}", pat_type.ty.to_token_stream().to_string()),
        })
        .collect::<Vec<String>>();
    let fn_ident = &signature.ident.to_string();
    let fn_result_type = &signature.output;

    let print_call = match fn_result_type {
        ReturnType::Default => format!("{}({})", fn_ident, fn_arg_types.join(", ")),
        ReturnType::Type(_, b) => format!("{}({}) -> {}", fn_ident, fn_arg_types.join(", "), b.to_token_stream().to_string()),
    };

    quote! {
        #(#attrs)*
        #visibility #signature {
            let __start_time = ::std::time::Instant::now();
            let __result = #block;

            println!("{}: {:?}", #print_call, __start_time.elapsed());
            __result
        }
    }
    .into()
}
