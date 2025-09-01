use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use syn::{parse_macro_input, LitInt, LitStr};
use quote::quote;

fn extract_items(input_val: &str) -> Vec<proc_macro2::TokenStream> {
    let mut items = Vec::with_capacity(input_val.len());

    for ch in input_val.chars() {
        if ch.is_ascii() {
            let item = Literal::byte_character(ch as u8);
            items.push(quote! {#item});
            continue;
        }

        let mut buf = [0; 4];
        ch.encode_utf8(&mut buf);

        for byte in buf.iter() {
            if *byte != 0 {
                let item_str = format!("0x{:x}", *byte);
                let item = LitInt::new(&item_str, Span::call_site());
                items.push(quote! {#item});
            }
        }
    }

    items
}

#[proc_macro]
pub fn has_prefix(ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(ts as LitStr);
    let input_val = input.value();

    if input_val.is_empty() {
        return TokenStream::from(quote! { [..] });
    }

    let items = extract_items(&input_val);

    TokenStream::from(quote! { [#(#items),*, ..] })
}

#[proc_macro]
pub fn has_suffix(ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(ts as LitStr);
    let input_val = input.value();

    if input_val.is_empty() {
        return TokenStream::from(quote! { [..] });
    }

    let items = extract_items(&input_val);

    TokenStream::from(quote! { [.., #(#items),*] })
}
