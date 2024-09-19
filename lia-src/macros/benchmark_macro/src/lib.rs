extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn benchmark(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let ItemFn { attrs, vis, sig, block } = input_fn;

    let output = quote! {
        #(#attrs)* #vis #sig {
            let _instant = std::time::Instant::now();
            let _result = (|| #block )();
            let benchmark_msg = format!("BENCHMARK<{}> = {:?}ms", stringify!(#sig), _instant.elapsed().as_millis());
            dbg!(benchmark_msg);
            _result
        }
    };

    TokenStream::from(output)
}
