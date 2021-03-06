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
        #[allow(dead_code,non_upper_case_globals)]
        #[test_case]
        pub const #ident: ::test2::TestDescAndFn = ::test2::TestDescAndFn {
            desc: ::test2::TestDesc {
                allow_fail: #allow_fail,
                ignore: #ignore,
                should_panic: ::test2::ShouldPanic::No,
                name: ::test2::StaticTestName(concat!(module_path!(), #test_path)),
            },
            testfn: ::test2::TestFn::StaticTestFn(|| {
                #spanned_func
                #ident()
            })
        };
    );
    res.into()
}