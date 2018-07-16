#![feature(rust_2018_preview)]
#![feature(proc_macro)]
#![feature(proc_macro_non_items)]
#![feature(test)]
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::ItemFn;
use syn::spanned::Spanned;

/* I think this is necessary due to a bug in macro hygiene */
use quote::*;

#[proc_macro_attribute]
pub fn test2(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut func: ItemFn = syn::parse(input).unwrap();
    let ident = &func.ident;
    let test_path = syn::LitStr::new(&format!("::{}", ident), proc_macro2::Span::call_site());

    let mut allow_fail = false;
    let mut ignore = false;
    func.attrs.retain(|ref attr| {
        match attr.path.clone().into_token_stream().to_string().as_ref() {
            "allow_fail" => allow_fail = true,
            "ignore" => ignore = true,
            _ => return true
        }
        false
    });

    let func_span = func.span();
    let spanned_func = quote_spanned!(func_span=> #func);
    let res = quote_spanned!(func_span=>
        #[test = "test2"]
        #[allow(dead_code,non_upper_case_globals)]
        const #ident: ::test::TestDescAndFn = ::test::TestDescAndFn {
            desc: ::test::TestDesc {
                allow_fail: #allow_fail,
                ignore: #ignore,
                should_panic: ::test::ShouldPanic::No,
                name: ::test::StaticTestName(concat!(module_path!(), #test_path)),
            },
            testfn: ::test::TestFn::StaticTestFn(|| {
                #spanned_func
                #ident()
            })
        };
    );
    res.into()
}